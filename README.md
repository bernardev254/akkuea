# Akkuea 🚀🎓

[![GitHub stars](https://img.shields.io/github/stars/akkuea/akkuea?style=social)](https://github.com/akkuea/akkuea)
[![GitHub issues](https://img.shields.io/github/issues/akkuea/akkuea)](https://github.com/akkuea/akkuea/issues)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/akkuea/akkuea/blob/main/LICENSE)

## 🌟 Welcome to Akkuea

Akkuea is not just a platform—it's a **global community** redefining the future of education. Powered by **open-source technology**, **AI**, and **blockchain**, Akkuea transforms how knowledge is accessed, created, and shared. Our mission is to make high-quality education **free**, **accessible**, and **collaborative** for everyone, everywhere. 🌍

Join us in building a **decentralized educational ecosystem** where educators, students, and creators are empowered, rewarded, and connected. Akkuea is a social network with **purpose**. ✨

---

## 🎯 Mission & Vision

### Mission

To create a collaborative platform where education is **free**, **accessible**, and **high-quality**, built by a diverse and motivated community. We empower those who teach, learn, and share while recognizing the value of every contribution.

### Vision

A world where anyone, regardless of background, can access and contribute to a **global knowledge library**. Akkuea aims to be the leading educational ecosystem for **collective, decentralized, and rewarded learning**.

---

## 🚀 Why Akkuea?

### The Problem

- 📚 **Scattered Resources**: Educational content is fragmented across platforms, making it hard to find and reuse.
- 🖼️ **Low Quality**: Many resources suffer from poor visuals or organization.
- 💸 **Lack of Incentives**: Educators and creators often go unrewarded for their efforts.
- 🌐 **Language Barriers**: Content is often limited to major languages.
- 🔒 **Proprietary Platforms**: Closed systems prioritize profit over impact.

### Our Solution

- **Centralized Hub**: A single platform with intelligent filters for educational content by level, language, and format.
- **AI-Powered Enhancement**: Automatic improvements to readability, visuals, and structure.
- **Transparent Rewards**: A **Stellar blockchain-based token system** that fairly rewards contributions.
- **Global Access**: Automatic translations, multi-format resources, and device compatibility.
- **Creator Marketplace**: Connect directly with designers for personalized resources, paid in tokens.

---

## 📚 Key Features

| Feature                       | Description                                                      |
| ----------------------------- | ---------------------------------------------------------------- |
| **Centralized Resources**     | Find and share educational materials in one place.               |
| **Rewards for Participation** | Earn tokens for contributing value to the community.             |
| **AI Visual Enhancement**     | Enhance documents and images for better educational impact.      |
| **Transparent Blockchain**    | Secure, traceable rewards via Stellar.                           |
| **Internal Marketplace**      | Request personalized resources from designers, paid with tokens. |
| **Automatic Translation**     | Multilingual access for global inclusivity.                      |
| **Multi-Platform**            | Fully functional on computers, tablets, and mobiles.             |
| **Accessibility**             | Easy-to-read, multi-format resources for all users.              |
| **Open Source**               | Developers worldwide can contribute to Akkuea’s growth.          |

---

## 👥 Key Users

- **Educators**: Share materials, access peer resources, and earn rewards for contributions.
- **Students**: Discover documents, quizzes, infographics, and join study groups.
- **Designers & Artists**: Create visuals, animations, and multimedia, earning credits and commissions.
- **AI Assistance**: Curates content, translates, simplifies, and enhances resources ethically.

---

## 💰 Reward System

Akkuea’s **Stellar blockchain** powers a transparent incentive system:

- **Earn Tokens**: Publish impactful content and receive tokens based on community engagement.
- **Bonuses**: Consistent contributors gain distinctions and extra rewards.
- **Traceable Credits**: Every AI-enhanced or collaborative resource tracks contributions.
- **Use Tokens**: Access marketplace services or withdraw for external use.

**Sharing knowledge = Creating value.** That value is now rewarded! 💡

---

## 🛠️ Technology Stack

- **Next.js**: Modern, fast, and scalable frontend framework.
- **Stellar Blockchain**: Secure and transparent reward system.
- **Generative & Analytical AI**: Powers translation, image enhancement, and content creation.
- **Open-Source Development**: Community-driven improvement for global collaboration.

---

## � PGetting Started

Ready to contribute to the future of education? Follow these steps to get Akkuea running on your local machine.

### Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js**: Version 20.11.0 or higher ([Download here](https://nodejs.org/))
- **Bun**: Version 1.0.25 or higher ([Installation guide](https://bun.sh/docs/installation))
- **Go**: Version 1.24.2 or higher ([Download here](https://golang.org/dl/))
- **PostgreSQL**: For the backend database ([Installation guide](https://www.postgresql.org/download/))
- **Git**: For version control ([Download here](https://git-scm.com/downloads))

> **💡 Tip**: We recommend using a version manager like [asdf](https://asdf-vm.com/) or [nvm](https://github.com/nvm-sh/nvm) to manage multiple Node.js versions.

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/akkuea/akkuea.git
   cd akkuea
   ```

2. **Install dependencies**
   ```bash
   bun install
   ```

3. **Set up environment variables**
   
   For the Go backend:
   ```bash
   cd packages/gin
   cp env.example .env
   ```
   
   Edit the `.env` file with your database credentials and configuration:
   ```env
   DB_HOST=localhost
   DB_USER=your_postgres_user
   DB_PASSWORD=your_postgres_password
   DB_NAME=akkuea
   DB_PORT=5432
   JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
   ```

4. **Set up the database**
   
   Create a PostgreSQL database:
   ```bash
   createdb akkuea
   ```
   
   Or using PostgreSQL CLI:
   ```sql
   CREATE DATABASE akkuea;
   ```

5. **Start the development servers**
   
   From the root directory:
   ```bash
   bun run dev
   ```
   
   This will start all services in parallel:
   - **Agent**: http://localhost:3000
   - **Assistant**: http://localhost:3001  
   - **Go API**: http://localhost:8080

### Individual Package Development

You can also run individual packages:

```bash
# Agent (Next.js frontend)
cd packages/agent
bun run dev

# Assistant (Next.js frontend)
cd packages/assistant
bun run dev

# Go API Backend
cd packages/gin
go run main.go
```

### Docker Setup (Alternative)

If you prefer using Docker:

```bash
cd packages/gin
docker-compose up -d
```

This will start the Go backend with PostgreSQL in containers.

### Verification

Once everything is running, you should be able to:

- ✅ Access the Agent frontend at http://localhost:3000
- ✅ Access the Assistant frontend at http://localhost:3001
- ✅ Make API calls to http://localhost:8080
- ✅ See database connections working without errors

### Common Issues & Troubleshooting

**Port conflicts**: If ports 3000, 3001, or 8080 are already in use, you can modify them in the respective package configurations.

**Database connection issues**: 
- Ensure PostgreSQL is running: `brew services start postgresql` (macOS) or `sudo systemctl start postgresql` (Linux)
- Verify your database credentials in the `.env` file
- Check if the database exists: `psql -l`

**Bun installation issues**: 
- On macOS: `curl -fsSL https://bun.sh/install | bash`
- On Windows: `powershell -c "irm bun.sh/install.ps1 | iex"`
- On Linux: `curl -fsSL https://bun.sh/install | bash`

**Go module issues**: Run `go mod tidy` in the `packages/gin` directory.

### Next Steps

- 📖 Check out our [Contributing Guide](CONTRIBUTING.md) for development guidelines
- 🐛 Report issues on [GitHub Issues](https://github.com/akkuea/akkuea/issues)
- 💬 Join our [Telegram community](https://t.me/akkuea_community) for support

---

## 📊 Project Status

🧱 **Under active development**  
We’re building the future of education, and we need your help! Contribute, leave feedback, or simply star the repo to show your support. 🌟

---

## 💬 Get Involved

- **Contribute**: Check out our [Contributing Guide](CONTRIBUTING.md) to get started.
- **Report Issues**: Open an issue on [GitHub](https://github.com/akkuea/akkuea/issues).
- **Join the Community**: Connect with us on our [Telegram group](https://t.me/akkuea_community).
- **Star the Repo**: Show your support by starring [Akkuea on GitHub](https://github.com/akkuea/akkuea)! ⭐

---

## 🫡 Thanks to Our Contributors

<a href="https://github.com/akkuea/akkuea/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=akkuea/akkuea" alt="Contributors" />
</a>

We’re grateful for every contribution that brings us closer to revolutionizing education! 🙌

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

**Join the educational revolution! 🚀🎓**  
**Happy Learning & Contributing! ✨**
