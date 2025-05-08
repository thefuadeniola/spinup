# Solana spinup
This installable binary helps to scaffold a new solana/web3 project using rust and typescript.

## Installatiion
In the directory where you want your solana projects:
```
cargo install solana-spinup
```

To spinup a new project:
```
solana-spinup new <PROJECT_NAME>
```

This spins up a new project with the sample file structure:
## file structure
```
├── program
| ├── cargo.toml
| └── src
|  └── lib.rs
├── client
| └── main.ts
└── package.json
```

It initializes and adds necessary dependencies including bosrh, mz, etc.
Since you can have multiple rust programs inside one solana project, to add another program to our project:
Inside the project directory you have created
```
solana-spinup add <NEW_PROGRAM_NAME>
```

## Scripts
This binary initializes the project with scripts to semi automate build, deployment and running the project. By combining npm and cargo commands into 1.

To compile the rust programs and place the executables inside a ./dist/program,
```
npm run build
```

To deploy the compiled .so rust files to the blockchain:
```
npm run deploy
```

To see a full list of executables such as `npm run reset and build` and `npm run clean`, check out this [package.json](https://github.com/thefuadeniola/solana_example_project/blob/main/package.json) and [cicd](https://github.com/thefuadeniola/solana_example_project/blob/main/scripts/cicd.sh) files.