# 40fy

40fy (fortify) is a desktop password manager application built to offer a simple, secure way to manage your passwords. Developed as a learning project in Rust and utilizing Tauri and SvelteKit, 40fy aims to provide a straightforward solution for password management across multiple vaults.
![dashboard](https://github.com/sandelit/40fy/blob/master/public/40fy.png)
![vaultlist](https://github.com/sandelit/40fy/blob/master/public/40fy2.png)

### Features

- Multiple Vaults: Organize your passwords into separate vaults, each protected by its master password for enhanced security.
- Secure Storage: All entries, including passwords and other sensitive information, are encrypted and securely stored in a SQLite database.
- Ease of Use: Designed with simplicity in mind, 40fy allows for easy addition and retrieval of entries, enabling quick copy-pasting of your credentials when needed.

### Getting Started

Currently, 40fy is in development and can be run locally by the following command:
```
npx tauri dev
```
In the future, 40fy will be available as a downloadable application (AppImage for Linux, EXE for Windows, etc.) suitable for various systems.
Contributing

### Disclaimer
This project is a learning endeavor, and as such, it may not meet all the security standards expected of a production-grade password manager. Please use it at your own risk.
