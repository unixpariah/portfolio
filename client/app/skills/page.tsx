import { Icons } from "@/components/icons";
import { Badge } from "@/components/ui/badge";

const categories = [
  {
    title: "Languages",
    items: [
      { title: "JavaScript", icon: "javascript" },
      { title: "nix", icon: "nixos" },
      { title: "TypeScript", icon: "typescript" },
      { title: "Rust", icon: "rust" },
      { title: "C", icon: "c" },
      { title: "Lua", icon: "lua" },
      { title: "Zig", icon: "zig" },
    ],
  },
  {
    title: "Frontend Development",
    items: [
      { title: "HTML", icon: "html" },
      { title: "CSS", icon: "css" },
      { title: "ReactJS", icon: "react" },
      { title: "NextJS", icon: "nextjs" },
      { title: "Tailwind CSS", icon: "tailwind" },
      { title: "shadcn/ui", icon: "shadcn" },
    ],
  },
  {
    title: "Backend Development",
    items: [
      { title: "NodeJS", icon: "nodejs" },
      { title: "ExpressJS", icon: "express" },
      { title: "actix-web", icon: "actix" },
    ],
  },
  {
    title: "Databases",
    items: [
      { title: "Postgres", icon: "postgresql" },
      { title: "sqlite", icon: "sqlite" },
    ],
  },
  {
    title: "DevOps & Tools",
    items: [
      { title: "Git", icon: "git" },
      { title: "GitHub", icon: "gitHub" },
      { title: "Docker", icon: "docker" },
      { title: "k3s", icon: "k3s" },
    ],
  },
  {
    title: "Package Managers",
    items: [
      { title: "npm", icon: "npm" },
      { title: "yarn", icon: "yarn" },
      { title: "pnpm", icon: "pnpm" },
      { title: "bun", icon: "bun" },
    ],
  },
  {
    title: "Operating Systems",
    items: [
      { title: "Linux", icon: "linux" },
      { title: "NixOS", icon: "nixos" },
    ],
  },
  {
    title: "Graphics & Low-Level",
    items: [
      { title: "OpenGL", icon: "opengl" },
      { title: "Vulkan", icon: "vulkan" },
      { title: "WGPU", icon: "wgpu" },
      { title: "libwayland", icon: "wayland" },
    ],
  },
  {
    title: "Tools & Editors",
    items: [
      { title: "neovim", icon: "neovim" },
      { title: "helix", icon: "helix" },
      { title: "mdbook", icon: "mdbook" },
    ],
  },
];

export default function SkillsPage() {
  return (
    <div>
      <div className="py-10">
        <h1 className="text-3xl font-bold">Skills &amp; Tools</h1>

        <p className="mt-4 text-muted-foreground">
          As a full‑stack Software Engineer, I specialize in building scalable
          web applications using modern technologies such as&nbsp;Next.js,
          React, and Tailwind&nbsp;CSS. I&rsquo;m also expanding my expertise
          into low‑level systems programming and graphics development with Rust,
          C, and&nbsp;Vulkan. My toolkit spans from frontend frameworks to
          infrastructure management with Docker and&nbsp;Kubernetes.
        </p>
      </div>

      <div className="space-y-12">
        {categories.map((category) => (
          <section key={category.title}>
            <h2 className="mb-4 text-xl font-semibold">{category.title}</h2>

            <div className="flex flex-wrap gap-2">
              {category.items.map((item) => (
                <Badge
                  key={item.title}
                  className="border border-secondary bg-gray-200 p-4 py-2 text-gray-800 dark:bg-gray-700 dark:text-gray-200"
                >
                  {Icons[item.icon as keyof typeof Icons]?.({
                    className: "mr-2 size-4",
                  })}
                  {item.title}
                </Badge>
              ))}
            </div>
          </section>
        ))}
      </div>
    </div>
  );
}
