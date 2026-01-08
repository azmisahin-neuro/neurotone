# Neurotone

**Neurotone**, bilimsel temelli **auditory neural entrainment**
(isochronic tones & binaural beats) üreten,
**Rust ile yazılmış** hafif bir CLI uygulamasıdır.

> ❗ Bu proje spiritüel / çakra / metafizik iddialar içermez.

---

## 🎯 Amaç
- Odak
- Meditasyon
- Rahatlama
- Uykuya geçiş

gibi durumlar için **beyin dalgası hedefli** ses üretimi.

---

## 🧠 Bilimsel Temel
Bu proje aşağıdaki kavramlara dayanır:

- EEG ile ölçülebilen **beyin dalgaları**
- **Neural Entrainment**
- Isochronic tones (en güçlü yöntemlerden biri)

Detaylar için: [`docs/SCIENCE.md`](docs/SCIENCE.md)

---

## 📦 Özellikler
- Alpha / Theta / Delta / Beta presetleri
- Isochronic tone üretimi
- `.wav` çıktı
- CLI kullanım
- Cross-platform

---

## 🚀 Kurulum
```bash
git clone https://github.com/azmisahin-neuro/neurotone.git
cd neurotone
cargo build --release
````

---

## ▶️ Kullanım

```bash
cargo run -- --mode delta --duration 600 --output docs/assets/samples/delta_2hz_600s.wav
cargo run -- --mode theta --duration 600 --output docs/assets/samples/theta_6hz_600s.wav
cargo run -- --mode alpha --duration 600 --output docs/assets/samples/alpha_10hz_600s.wav
cargo run -- --mode beta --duration 600 --output docs/assets/samples/beta_18hz_600s.wav

```

### Modlar

| Mod   | Hz | Amaç       |
| ----- | -- | ---------- |
| delta | 2  | Derin uyku |
| theta | 6  | Meditasyon |
| alpha | 10 | Sakin odak |
| beta  | 18 | Dikkat     |

---

## 🎧 Güvenlik

* Ses seviyesini **düşük** tutun
* Uzun süre yüksek sesle dinlemeyin
* Epilepsi geçmişi olanlar kullanmamalıdır

Detaylar için: [`docs/DISCLAIMER.md`](docs/DISCLAIMER.md)

---

## 🛠️ Teknoloji

* Rust
* hound (WAV output)
* clap (CLI)

---

## 🎧 Sample Outputs
Pre-generated example outputs are available under:

`assets/samples/`

These files demonstrate the default presets produced by Neurotone.

