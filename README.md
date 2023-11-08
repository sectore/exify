# exify

<p float="left">
  <a href="wiki/exify-1.png" target="_blank">
    <img src="wiki/exify-1.png" width="30%" />
  </a>
  <a href="wiki/exify-2.png" target="_blank">
    <img src="wiki/exify-2.png" width="30%" />
  </a>
  <a href="wiki/exify-3.png" target="_blank">
    <img src="wiki/exify-3.png" width="30%" />
  </a>
</p>

Web app to remove [EXIF](https://en.wikipedia.org/wiki/Exif) (Exchangeable Image File Format) data from images. 


All processing is done in the browser. No server. No data is sent anywhere. It still works offline.


## Usage

Open https://bitcoinbeachtravemuende.github.io/exify/ in your browser. 

## Development

### Prerequisites

#### Nix (recommended)

Install [Nix](https://zero-to-flakes.com/install)

#### Other

Install [Rust](https://www.rust-lang.org/tools/install) and [Trunk](https://trunkrs.dev/)


### Build from source

`cd` into the project directory and run:

### Nix

```bash
nix develop
trunk build --release
```

### Or others

```bash
trunk build --release
```


### Developing locally

`cd` into the project directory and run:

### Nix

```bash
nix develop
trunk serve
```

### Or others

```bash
trunk serve
```

Open browser at http://127.0.0.1:8080/exify

## FAQ

### What does `exify` mean?

The name `exify` is derived from EXIF, the file format used to store metadata in images.

### Do I need an Internet connection?

No. The application works offline.

### What happens to my original images?

Original images will be unchanged. All changes will be saved as a new image prefixed with `exify_`.

### How does `exify` work?

Technically the application is built with [Yew](https://yew.rs/) / ([Rust](https://www.rust-lang.org/)) and compiled to [WebAssembly](https://webassembly.org/). It uses [kamadak-exif](https://crates.io/crates/kamadak-exif) and [img-parts](https://crates.io/crates/img-parts) crates to parse and remove EXIF data from images.

All code runs in the browser. No server is needed. 

### What browsers are supported?

All modern browsers.

### What image formats are supported?

`jpg`, `png` and `webp` formats are supported.

### What metadata is removed?

All EXIF data recognized by the application will be removed.

### Is this application free?

Yes. The source code is available on GitHub under the MIT license.

### How can I contribute?

You can contribute by reporting bugs, suggesting features or by submitting pull requests.


## License

[MIT Lizenz](./LICENSE)
