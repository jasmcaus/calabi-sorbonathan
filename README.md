# Delta

# Soroban Stuff

Public key: `GAIM4S6HDNGDW4R7452E3SGOSZ3UOAXXGHXEILCDRS4U7KZHRQAMJVOZ`

```
Local deployments:
Asset: CC7Q7J4MJDEAXNSIWKGZZLTGM5J7SQ23V4LGLU5WOWVV4B7HYM7CGWKW

Futurenet deployments:
Contract: CDXJNGQNV66Q2DXAGBUSGWGFV6YKZU4EGDY4ZBRZHFTKMPMMQHTNWWFE
```

```bash
sudo apt-get update && sudo apt-get install ffmpeg libsm6 libxext6  -y

rustup target add wasm32-unknown-unknown
cargo install --locked --version 20.0.0-rc2 soroban-cli --features opt
echo "source <(soroban completion --shell bash)" >> ~/.bashrc

cargo new --lib contracts

soroban config network add testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"
soroban config identity generate alice

# Build
soroban contract build && soroban contract optimize --wasm target/wasm32-unknown-unknown/release/contracts.wasm

# Deploy
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/contracts.optimized.wasm --source alice --network testnet > ../contractId

# Invoke on Local testnet
soroban contract invoke --wasm target/wasm32-unknown-unknown/release/contracts.wasm --id 1 -- hello --to friend

# Invoke on Futurenet
soroban contract invoke --id $(cat ../.contractId) --source alice --network testnet -- hello --to RPC

---

# Frontend
cargo install_soroban
chmod +x soroban

soroban contract bindings typescript --network testnet --contract-id $(cat contracts/.soroban/hello-id) --output-dir amorphous-soroban-client
```


```
Deployments:
- Token: CCA4MRQO4N6RYK7IJIOMYLREA3AJNXTRJTVT2ETML2MGDGTU3322OROK

```

```md
Token deployments:
make optimize
soroban contract deploy --wasm contracts/target/wasm32-unknown-unknown/release/token.optimized.wasm --source alice

# Initialize
soroban contract invoke --wasm contracts/target/wasm32-unknown-unknown/release/token.wasm --id 1 -- initialize --admin GAIM4S6HDNGDW4R7452E3SGOSZ3UOAXXGHXEILCDRS4U7KZHRQAMJVOZ --decimal 6 --name "USDT" --symbol "USDT"

# Mint
soroban contract invoke --wasm contracts/target/wasm32-unknown-unknown/release/token.wasm --id 2 -- mint --to GBLPQPPE7B5DFNNR5FKIVIXJCRZUEBNRHMNHCLZZYNZV7DP6WZUS2LED --amount "5"
```


# Astro Starter Kit: Basics

```sh
npm create astro@latest -- --template basics
```

[![Open in StackBlitz](https://developer.stackblitz.com/img/open_in_stackblitz.svg)](https://stackblitz.com/github/withastro/astro/tree/latest/examples/basics)
[![Open with CodeSandbox](https://assets.codesandbox.io/github/button-edit-lime.svg)](https://codesandbox.io/p/sandbox/github/withastro/astro/tree/latest/examples/basics)
[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/withastro/astro?devcontainer_path=.devcontainer/basics/devcontainer.json)

> 🧑‍🚀 **Seasoned astronaut?** Delete this file. Have fun!

![just-the-basics](https://github.com/withastro/astro/assets/2244813/a0a5533c-a856-4198-8470-2d67b1d7c554)

## 🚀 Project Structure

Inside of your Astro project, you'll see the following folders and files:

```text
/
├── public/
│   └── favicon.svg
├── src/
│   ├── components/
│   │   └── Card.astro
│   ├── layouts/
│   │   └── Layout.astro
│   └── pages/
│       └── index.astro
└── package.json
```

Astro looks for `.astro` or `.md` files in the `src/pages/` directory. Each page is exposed as a route based on its file name.

There's nothing special about `src/components/`, but that's where we like to put any Astro/React/Vue/Svelte/Preact components.

Any static assets, like images, can be placed in the `public/` directory.

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

## 👀 Want to learn more?

Feel free to check [our documentation](https://docs.astro.build) or jump into our [Discord server](https://astro.build/chat).
