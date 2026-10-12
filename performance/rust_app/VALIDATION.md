# Проверка единого numbered-стенда 0.2.0

Проверено 12 октября 2026 года. Локальная iOverlay 9.0.1, revision `7a9e659a0355af272cb26383be169557a78e5ea4`; библиотека не изменена. Rust `1.99.0 (b940084d7 2026-09-28)`, target `aarch64-apple-darwin`, macOS.

- Прежние файлы и номера 0..8 сохранены. Исходные integer-генераторы закреплены golden fingerprints; исходные float-генераторы сохранены с предварительной конвертацией для выбранного integer engine.
- Новые файлы: test_9_orientation, test_10_diagonal, test_11_collinear, test_12_cluster, test_13_seeded. Дублирование старых типов нагрузки удалено. Linear-layout — вариант test_1; integer-спираль — вариант test_3.
- Единственный основной бинарник принимает старые --test/--count/--complex/--int; legacy и отдельный named-case runner удалены.
- `cargo test --release --offline --locked --all-features`: 9 тестов прошли. Все номера и варианты на размерах 1/4/8, все три integer engine и четыре стратегии, reinit, аналитические площади, поворот, coordinate budget, PRNG-векторы, seeded/старые integer fingerprints проверены.
- `cargo clippy --release --offline --locked --all-targets --all-features -- -D warnings`: без предупреждений.
- `results/smoke-numbered.csv`: 384 строки — 16 вариантов номеров 0..13 × 3 engines × 4 strategies × 2 modes. count=8, seed=0, warmup=1, samples=3, iterations=1.
- `results/smoke-numbered-mt.csv`: те же 384 комбинации, включённая feature multithreading, runtime=true, RAYON_NUM_THREADS=4. Все проверки прошли; input fingerprints и сводки результатов совпали с serial-прогоном. Малые размеры не доказывают фактического parallel sort или ускорения.
- `results/baseline.csv`: новый baseline schema=2/fixture_version=2, 128 строк; default count=8, int=i32, warmup=2, samples=7, iterations=3.
- `results/thresholds.csv`: 20 строк, test_1 layout=line, 3992/4000/8000/16000/16008 исходных рёбер, i32, четыре стратегии, cold; результаты совпали. При 16008 route hint запрошенных Tree и Frag одинаков: Frag.
- `results/variants.csv`: 48 строк, test_1 layout=line и test_3 spiral=integer, count=32, все engines/strategies/modes; проверки прошли.
- CSV прочитаны Python csv.DictReader: ожидаемое число строк, schema/fixture_version, структура полей, validated=true, число выборок, медиана/min/max проверены.
- `git diff --check`: без ошибок. Все изменения ограничены performance/rust_app; examples и библиотека не изменены. Commit/push не выполнялись.

Старые результаты/описания предыдущей организации сохранены в локальном `results/previous-layout/` и не являются baseline новой матрицы. Каталог results игнорируется Git. Команды запуска и ограничения (float, soft budget, диапазон, маршрутизация Tree→Frag) описаны в README.
