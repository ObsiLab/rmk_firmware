<img align="left" height="120" src="https://user-images.githubusercontent.com/23436953/178232652-e7b1504c-0425-490a-ab8c-12a58e288097.png">

# RMK - a Rust Keyboard Firmware ⌨️
Originally made for the [Quanta keyboard](https://github.com/ObsiLab/Quanta).
## _**----- 🏗️ THIS IS A WORK IN PROGRESS 🚧 -----**_  

<details>
  <summary><h2>Table Of Contents 📑</h2></summary>
 
>   - [📖 Docs](#-docs)
>     - [▶️ Getting started](#%EF%B8%8F-getting-started)
>       - [_yes_](#yes-)
>   - [🔡 Details](#-details)
>     - [📝 Authors and Contributors](#-authors-and-contributors)
>       - [Author](#author-)
>       - [Contributors](#contributors-)
>     - [🌟 Acknowledgements](#-acknowledgements)
>     - [🧑‍🤝‍🧑 Contributing](#-contributing)
>     - [®️ License](#%EF%B8%8F-license)
 
</details>

# 📖 Docs
https://rmk.obsilab.com

## ▶️ Getting started
### _yes_ :
_WIP_

### Keymap
Make a keymap.json file _manually_, based on [example_keymap.json](example_keymap.json).
```json
{
  "name": "Example Keymap",
  "version": 1.0,
  "author": "RMK",
  "layers": 2,
  "max_rows": 3,
  "max_columns": 3,
  "description": "Experimental. An example keymap for RMK firmware, 3x3 key matrix with two layers.",
	"layer1": {
		"row1": ["KEY_Q", "KEY_W", "KEY_E"],
		"row2": ["KEY_A", "KEY_S", "KEY_D"],
		"row3": ["KEY_null", "KEY_null", "KEY_null"]
	},
	"layer2": {
		"row1": ["KEY_TRNS", "KEY_T", "KEY_MEDIA_PLAY_PAUSE"]
	}
}
```  
Or use **[RMK GUI Configurator (RGC)](https://github.com/ObsiLab/RGC)** (_WIP_).

-----------------

# 🔡 Details

## 📝 Authors and Contributors
### Author :
- [Lucas Placentino](https://github.com/LucasPlacentino)
### Contributors :
- [List of contributors](../../graphs/contributors)

## 🌟 Acknowledgements
#### 💡 Inspired by [**QMK**](https://github.com/qmk/qmk_firmware) and [**KMK**](https://github.com/KMKfw/kmk_firmware).

#### 🧱 Based off of:
- https://github.com/Innectic/rmk
- https://github.com/rp-rs/rp-hal
- https://github.com/dlkj/usbd-human-interface-device
- https://github.com/TeXitoi/keyberon
- https://github.com/camrbuss/pinci
- https://github.com/ChrisChrisLoLo/keezyboost40/tree/master/firmware/keezus

## 🧑‍🤝‍🧑 Contributing
_[...](CONTRIBUTING.md)_

## ®️ License
Licensed under an [**MIT License**](LICENSE)

-------------------

> _[↑ Go To TOP](#TOP)_
