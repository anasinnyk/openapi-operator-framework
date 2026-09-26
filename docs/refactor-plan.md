# План рефакторингу koof

Мета: **мінімум власного коду в ядрі**. Не просто менше рядків, а менше логіки,
яку ми самі підтримуємо. Усе, що вміють готові бібліотеки (моделі OpenAPI, генерація
Rust-типів, HTTP-клієнт), віддаємо їм. Ядро лишається тільки для Kubernetes-специфіки:
overlay → IR, фільтрація полів, посилання, credentials, reconcile.

Кожна фаза — окремий PR із чіткими критеріями приймання. Фази 0–1 не змінюють поведінку.
Фаза 2 — спайк без production-коду. Після кожної фази — контрольна точка (✋): рев'ю
і рішення, чи йдемо далі.

---

## Прийняте рішення: roas + typify + generic `Operation`, без progenitor

- **roas** — завантаження, валідація і нормалізація специфікації (2.0 / 3.0 / 3.1 / 3.2)
  до 3.1. Апгрейд 3.0 → 3.1 переписує `nullable: true` у `type: [T, "null"]`, тож
  `components.schemas` стають звичайною JSON Schema.
- **roas-overlay** — застосування overlay (без змін).
- **typify** — Rust-типи з `components.schemas`. `$ref` вигляду
  `#/components/schemas/X` typify розв'язує за останнім сегментом, тож схеми
  передаються в `add_ref_types` без змін. Обмеження: модель typify —
  `schemars 0.8` (draft-07); ключові слова лише з 2020-12 (`prefixItems`,
  `unevaluatedProperties`) ігноруються.
- **HTTP-клієнт не генерується.** У runtime є trait `koof::Operation`
  (`METHOD`, `PATH`, `Request`, `Response`) і один generic `HttpClient::send::<O>`
  (auth, url-encode path-параметрів, 404 → `None`, інший не-2xx → помилка з тілом).
  Генератор видає тільки `impl Operation` з даними й посиланнями на typify-типи.

Чому не progenitor: він працює тільки з `openapiv3` (3.0), а roas конвертує лише вгору.
Крім того, адаптери під його модель (inner type, hooks, мапінг `Error`) все одно були б
нашим кодом, тільки менш очевидним. Усе, що дає progenitor поверх цього (query/header
параметри, typed errors, builder-и), оператору не потрібне. Query-параметри додамо
полем `QUERY` в `Operation`, коли вони знадобляться.

---

## Фаза 0 — зелений baseline і golden-перевірка ✅

Без цього не видно, що зламала кожна наступна фаза.

- `FieldSelectionModeIr`: додати `#[serde(rename_all = "snake_case")]` (зараз падає
  регенерація на `mode: explicit`).
- Оновити `fixtures/minimal-provider.yaml` під `field_selection`.
- Прибрати `unwrap`/`expect` з тестів і `expect("validated HTTP method")` зі
  згенерованого клієнта. Закрити `# Errors` та інші pedantic lint-и.
- Скрипт `scripts/regen.sh`: повна регенерація Cloudflare + `cargo fmt` для generated.
- CI-перевірка / тест: регенерація в temp-директорію → `diff` з tracked `src/generated/`
  (перевіряє детермінованість і що generated-код не відстає від генератора).

**Приймання:** `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets -- -D warnings` і `scripts/regen.sh && git diff --exit-code` — зелені.

✋ Контрольна точка.

---

## Фаза 1 — розбиття на два крейти (тільки переміщення коду) ✅

| Крейт | Роль | Що туди йде | Залежності |
|---|---|---|---|
| `koof` | runtime, від нього залежать провайдери | `api`/`http` (креди), `reference`, `managed`, `reconciler`, `error` | kube, reqwest, serde |
| `koof-gen` | build-time, lib + bin | `cli`, `loader`, `extractor`, `ir`, `validation`, `generator` | roas-overlay, quote, syn, prettyplease, clap (далі roas, typify) |

Навіщо:
- Межа «що працює в операторі» / «що працює при генерації» стає видимою в `Cargo.toml`.
  Runtime не може випадково залежати від codegen, і навпаки.
- Провайдер перестає компілювати `syn`/`clap`/`roas-overlay`, а потім і `typify`.
- Згенерований код посилається тільки на `koof::…`, тож публічний API runtime
  природно стає контрактом для генератора.

Третій крейт (окремий `koof-cli`) поки не потрібен: bin у `koof-gen` достатньо.

**Приймання:** generated-файли побайтово ті самі (`git diff --exit-code` після регенерації),
усі перевірки з фази 0 зелені.

✋ Контрольна точка.

---

## Фаза 2 — спайк roas + typify (timebox, окрема гілка, не мерджиться)

Відповісти на питання, від яких залежать фази 3–4:

1. Чи зберігає roas `x-kube-*` extensions при parse → upgrade до 3.1 → serialize?
   Чи витримує roas-валідація Cloudflare-специфіку (478k рядків) і скільки часу займає?
2. typify на **обрізаних** `components.schemas` (досяжних з `x-kube-operation`):
   чи компілюється результат, скільки типів і рядків, які імена
   (`DnsRecordsDnsRecordPost` тощо), чи потрібні `with_patch` / `with_rename`.
3. Чи компілюється `with_derive("schemars::JsonSchema")`, якщо в провайдері `schemars 1.x`
   (typify за замовчуванням не генерує `#[schemars(...)]`-атрибутів, тож очікую «так»).
4. Чи проходить CRD, згенерована `kube::CustomResource` з typify-типів для `forProvider`,
   structural-перевірку (`kubectl apply --dry-run=server` у Kind)? Особливо `anyOf`
   у DNSRecord (`dns-records_dns-record-post` = `anyOf[without-data, with-data]`).
5. Прототип `koof::Operation` + `HttpClient::send` на Account observe.

**Результат:** короткі нотатки в цьому файлі + go/no-go для фази 4.
Якщо п.4 не проходить → додається фаза 6 (див. нижче).

✋ Контрольна точка.

---

## Фаза 3 — препроцесинг специфікації + фільтрація полів

Уся трансформація робиться **на рівні JSON до codegen**. Генератор типів отримує
вже готову специфікацію і нічого не знає про Kubernetes.

Pipeline у `koof-gen`:

```
load YAML → apply overlays (roas-overlay)
          → roas: parse + validate + upgrade до 3.1
          → extract x-kube-* → ResourceIr / OperationIr (без SchemaIr)
          → validate
          → prune: лишити тільки paths з x-kube-operation + досяжні $ref-схеми
          → synthesize: для кожного ресурсу додати в components.schemas
               <Kind>ForProvider  і  <Kind>AtProvider
          → components.schemas (JSON Schema) + OperationIr → codegen
```

Синтез схем (тут реалізується фільтрація, яка зараз не працює):
- Пошук властивості за ім'ям: обхід `$ref` / `allOf` / `anyOf` / `oneOf` / `properties`,
  повертає першу знайдену схему властивості. Це заміняє `object_properties` +
  `merge_field` + `distribute_all_of` і не потребує загального «злиття allOf».
- `mode: explicit` — беремо тільки перелічені `forProvider` / `atProvider` поля.
- `mode: all` — усі знайдені властивості; для `forProvider` мінус `readOnly` і
  `x-kube-field.ignored`.
- `required` для `forProvider` — перетин з `required` джерела.
- `atProvider` береться з response-схеми observe-операції з урахуванням envelope.
  Нове поле в overlay: `x-kube-operation.response.path: result` (Cloudflare `{ result: … }`).
- Reference-поля (`zoneRef`) і credential-поля в ці схеми **не** додаються: їх додає
  генератор CRD як `koof::reference::*`.

На цій фазі старий codegen ще працює (читає синтезовані схеми через поточний `SchemaIr`),
щоб фільтрацію можна було перевірити окремо від заміни генератора.

**Тести:** маленькі fixture-специфікації (не Cloudflare): explicit/all, readOnly, ignored,
поле всередині allOf/anyOf, envelope `result`, невідоме поле в explicit-списку → помилка
з location.

**Приймання:** перевірки з фази 0 зелені; `client.rs` різко зменшився через prune;
у `DNSRecordForProvider` рівно ті поля, що в overlay.

✋ Контрольна точка.

---

## Фаза 4 — typify + generic `Operation` замість власного codegen

- `koof-gen`: `components.schemas` препроцесованої специфікації → `typify::TypeSpace`
  (`with_derive("schemars::JsonSchema")`, `PartialEq`) → `src/generated/types.rs`.
- `koof-gen`: для кожної операції — `pub struct <Resource><Lifecycle>;` +
  `impl koof::Operation` (`METHOD`, `PATH`, `Request`, `Response`) → `src/generated/operations.rs`.
  Жодних тіл функцій.
- CRD: `<Kind>Spec { for_provider: types::<Kind>ForProvider, … }` — тип синтезованого
  компонента, окремої генерації не потрібно.
- `koof` runtime (нове, мале, покрите тестами):
  - `koof::Operation` trait;
  - `koof::http::HttpClient::send::<O>(auth, params, body) -> Result<Option<O::Response>>`
    (404 → `None`, не-2xx → `HttpError` зі статусом і тілом);
  - `koof::http::Auth` — enum `Bearer` / `Header { name, value }`; секрети не
    потрапляють у `Debug` і логи;
  - `koof::http::extract(value, "/result")` — envelope → `AtProvider`.
- Генерація `<Scheme>Credential`-типів зникає: `SecuritySchemeIr` мапиться на варіант
  `Auth` даними, а не кодом.

**Видаляється:** `generator/schema.rs` повністю (разом із `generate_http_client`),
`SchemaIr`/`FieldIr`/`AdditionalPropertiesIr` з `ir.rs`, `extract_schema*` /
`extract_request_body` / `extract_response_body` з extractor, `koof::api::ApiRequest`,
генерація credential-типів.

**Приймання:** перевірки зелені; observe для Account працює в Kind як і раніше;
diff generated переглянутий (нові імена типів — очікувано).

✋ Контрольна точка.

---

## Фаза 5 — generic runtime замість `quote!`-логіки

Зараз `resource.rs` генерує для кожного ресурсу однаковий код: обхід посилань,
`resolve_credentials`, `update_status`, `reconcile`, `run_controller`. Переносимо це
в `koof` як звичайний (тестований) Rust, а генеруємо тільки дані й тонкий клей.

- `koof::ManagedResource` trait: `descriptor()` зі статичними даними (identifiers,
  references, credential source, field paths) + асоційовані `ForProvider` / `AtProvider`.
- Асоційовані типи `ManagedResource::Observe` / `Create` / `Update` / `Delete: Operation`
  (з фази 4). Generic `reconcile::<R>` викликає `client.send::<R::Observe>(…)`, тож
  згенерованих тіл функцій не лишається зовсім.
- Generic `koof::reconcile::<R>`, `koof::run_controller::<R>`, `koof::status::update::<R>`.
- Обхід посилань і `via`-ланцюжків — runtime через `kube::Api<DynamicObject>` +
  `ApiResource` з descriptor-а і JSON-pointer по полях (замість `field_path_access`,
  що генерує `.a.b.c`). Політика namespace застосовується явно в одній функції.
- Credentials: `koof::credentials::resolve::<R>(client, &resource) -> Auth` — один
  generic шлях `Related { via } → SecretKeySelector`.

**Видаляється:** більша частина `generator/resource.rs` (очікую ~1086 → ~200 рядків).

**Тести:** unit-тести на traversal / namespace policy / credentials з fake kube-client
(`tower-test`) замість diff-перевірок згенерованого коду.

✋ Контрольна точка.

---

## Фаза 6 (умовна) — CRD-схема напряму з OpenAPI

Тільки якщо спайк (п.3) покаже, що schemars з typify-типів дає non-structural схему.

- `#[kube(schema = "disabled")]` для CRD-типу, а `openAPIV3Schema` генерується з
  синтезованої `<Kind>ForProvider` (inline `$ref`, `anyOf`/`oneOf` → злиття властивостей
  або `x-kubernetes-preserve-unknown-fields`).
- `crdgen` бере YAML зі згенерованого файлу.

---

## Поза межами цього плану

Create / update / delete, finalizers, `managementPolicies` / `deletionPolicy`, drift
detection, late initialization. Їх краще будувати вже на фазах 4–5: тоді це буде
логіка в `koof::reconcile`, а не новий `quote!`.

## Очікуваний результат

| | Зараз | Після фаз 0–5 (оцінка) |
|---|---|---|
| Власний codegen типів і клієнта | `schema.rs` 729 + SchemaIr ~100 + extractor ~250 | ~60 (emit `impl Operation`), типи — typify |
| Препроцесинг (prune + synthesize) | — | ~200–250 |
| `resource.rs` | 1086 | ~200 |
| Runtime `koof` | ~535 | ~700–900, але звичайний тестований Rust |
| Крейти | 1 змішаний | `koof` (runtime) + `koof-gen` (build-time) |
