export const dynamic = "auto";

import { siteConfig } from "@/config/site";
import Link from "next/link";

// marking this as async for now
// maybe we can make this component dynamic with revalidate time in the future
export async function SiteFooter() {
  const currentYear = new Date().getFullYear();
  return (
    <footer className="py-2 border-t border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container-wrapper">
        <div className="container flex flex-col items-center justify-center py-4 space-y-2">
          <div className="max-w-3xl text-xs leading-relaxed text-center sm:text-sm text-balance text-muted-foreground">
            <span className="block sm:inline">© 2025 - {currentYear}</span>
            <span className="hidden sm:inline"> · </span>
            <span className="block sm:inline">
              Built with{" "}
              <span className="font-medium text-foreground">Next.js</span>,{" "}
              <span className="font-medium text-foreground">shadcn/ui</span>,{" "}
              <span className="font-medium text-foreground">Tailwind CSS</span>{" "}
              and <span className="font-medium text-foreground">Rust</span>
            </span>
            <span className="hidden sm:inline"> · </span>
            <span className="block sm:inline">
              Coded in{" "}
              <span className="font-medium text-foreground">helix</span> and
              deployed locally on my{" "}
              <span className="font-medium text-foreground">
                Raspberry Pi 5
              </span>
            </span>
          </div>
          <div className="text-xs text-center sm:text-sm text-muted-foreground">
            Developed by{" "}
            <Link
              href={siteConfig.links.githubProfile}
              target="_blank"
              rel="noreferrer"
              className="font-medium transition-colors text-primary hover:text-primary/80"
            >
              Oskar Rochowiak
            </Link>{" "}
            · Source code available on{" "}
            <Link
              href={siteConfig.links.github}
              target="_blank"
              rel="noreferrer"
              className="font-medium transition-colors text-primary hover:text-primary/80"
            >
              GitHub
            </Link>
          </div>
        </div>
      </div>
    </footer>
  );
}
