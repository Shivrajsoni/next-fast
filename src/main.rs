use clap::Parser;
use colored::*;
use std::env;
use std::process::{Command, Stdio};
use std::path::Path;

#[derive(Parser)]
#[command(name = "next-fast")]
#[command(about = "Create a Next.js app with bun and initialize Prisma")]
#[command(version = "0.1.0")]
struct Cli {
    /// Name of the project
    project_name: String,
    
    /// Use TypeScript (default: true)
    #[arg(long, short, default_value_t = true)]
    typescript: bool,
    
    /// Use Tailwind CSS
    #[arg(long, default_value_t = true)]
    tailwind: bool,
    
    /// Use ESLint
    #[arg(long, default_value_t = true)]
    eslint: bool,
    
    /// Use App Router (default: true)
    #[arg(long, default_value_t = true)]
    app: bool,

    /// Skip package manager selection prompt
    #[arg(long, default_value_t = false)]
    skip_install: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    println!("{}", "🚀 Creating Next.js app with Prisma...".bright_blue().bold());
    
    // Check if bun is installed
    check_bun_installed()?;
    
    // Create Next.js app with bun
    create_nextjs_with_bun(&cli).await?;
    
    // Initialize Prisma
    initialize_prisma(&cli.project_name).await?;
    
    // shadcn creation
    initialize_shadcn().await?;

    // Show completion message
    show_completion_message(&cli.project_name);
    
    Ok(())
}

fn check_bun_installed() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔍 Checking for bun...".yellow());
    
    let output = Command::new("bun")
        .arg("--version")
        .output();
    
    match output {
        Ok(_) => {
            println!("{}", "✅ bun found!".green());
            Ok(())
        }
        Err(_) => {
            eprintln!("{}", "❌ bun is not installed or not in PATH".red());
            eprintln!("{}", "Please install bun from: https://bun.sh".yellow());
            std::process::exit(1);
        }
    }
}

async fn create_nextjs_with_bun(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📦 Creating Next.js app with bun...".yellow());
    
    let mut cmd = Command::new("bun");
    cmd.arg("create")
       .arg("next-app")
       .arg(&cli.project_name);
    
    // Add TypeScript flag
    if cli.typescript {
        cmd.arg("--typescript");
    } else {
        cmd.arg("--javascript");
    }
    
    // Add Tailwind flag
    if cli.tailwind {
        cmd.arg("--tailwind");
    } else {
        cmd.arg("--no-tailwind");
    }
    
    // Add ESLint flag
    if cli.eslint {
        cmd.arg("--eslint");
    } else {
        cmd.arg("--no-eslint");
    }
    
    // Add App Router flag
    if cli.app {
        cmd.arg("--app");
    } else {
        cmd.arg("--no-app");
    }
    
    // Skip package manager selection
    if cli.skip_install {
        cmd.arg("--skip-install");
    }
    
    // Execute the command
    let status = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if !status.success() {
        eprintln!("{}", "❌ Failed to create Next.js app".red());
        std::process::exit(1);
    }
    
    println!("{}", "✅ Next.js app created successfully!".green());
    Ok(())
}

async fn initialize_prisma(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🗄️ Initializing Prisma...".yellow());
    
    // Change to project directory
    let project_path = Path::new(project_name);
    env::set_current_dir(project_path)?;
    
    // Add Prisma dependencies
    println!("{}", "📦 Adding Prisma dependencies...".cyan());
    let status = Command::new("bun")
        .args(&["add", "prisma", "@prisma/client"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if !status.success() {
        eprintln!("{}", "❌ Failed to add Prisma dependencies".red());
        std::process::exit(1);
    }
    
    // Initialize Prisma
    println!("{}", "🔧 Initializing Prisma schema...".cyan());
    let status = Command::new("bunx")
        .args(&["prisma", "init", "--datasource-provider", "sqlite"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if !status.success() {
        eprintln!("{}", "❌ Failed to initialize Prisma".red());
        std::process::exit(1);
    }
    
    // Create a basic schema with example models
    create_basic_schema().await?;
    
    // Generate Prisma client
    println!("{}", "⚡ Generating Prisma client...".cyan());
    let status = Command::new("bunx")
        .args(&["prisma", "generate"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if !status.success() {
        eprintln!("{}", "❌ Failed to generate Prisma client".red());
        std::process::exit(1);
    }
    
    println!("{}", "✅ Prisma initialized successfully!".green());
    Ok(())
}

async fn create_basic_schema() -> Result<(), Box<dyn std::error::Error>> {
    let schema_content = r#"// This is your Prisma schema file,
// learn more about it in the docs: https://pris.ly/d/prisma-schema

generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "sqlite"
  url      = env("DATABASE_URL")
}

model User {
  id        Int      @id @default(autoincrement())
  email     String   @unique
  name      String?
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  posts     Post[]
}

model Post {
  id        Int      @id @default(autoincrement())
  title     String
  content   String?
  published Boolean  @default(false)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  author    User     @relation(fields: [authorId], references: [id])
  authorId  Int
}
"#;
    
    tokio::fs::write("prisma/schema.prisma", schema_content).await?;
    println!("{}", "📝 Created basic Prisma schema with User and Post models".green());
    Ok(())
}

async fn initialize_shadcn()->Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📦 Initalizing Shadcn...".yellow());
    let mut cmd = Command::new("bunx");
    cmd.arg("shadcn@latest")
       .arg("init");

     // Execute the command
    let status = cmd
     .stdout(Stdio::inherit())
     .stderr(Stdio::inherit())
     .status()?;
 
    if !status.success() {
     eprintln!("{}", "❌ Failed to initalizes Shadcn ".red());
     std::process::exit(1);
    }
 
    println!("{}", "✅ Shadcn Initializes Successfully !".green());
    Ok(())
}


fn show_completion_message(project_name: &str) {
    println!("\n{}", "🎉 Project created successfully!".bright_green().bold());
    println!("\n{}", "Next steps:".bright_blue().bold());
    println!("  1. {}", format!("cd {}", project_name).cyan());
    println!("  2. {}", "bunx prisma db push".cyan());
    println!("  3. {}", "bun dev".cyan());
    println!("\n{}", "Prisma commands:".bright_blue().bold());
    println!("  • {}: {}", "Push schema to database".yellow(), "bunx prisma db push".cyan());
    println!("  • {}: {}", "Open Prisma Studio".yellow(), "bunx prisma studio".cyan());
    println!("  • {}: {}", "Generate client".yellow(), "bunx prisma generate".cyan());
    println!("  • {}: {}", "Create migration".yellow(), "bunx prisma migrate dev".cyan());
    println!("\n{}", "Happy coding! 🚀".bright_magenta());

}