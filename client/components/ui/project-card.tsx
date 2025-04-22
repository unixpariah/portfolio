import { Language, Project } from "@/app/projects/page";

interface ProjectCardProps {
  project: Project;
}

function langColor(lang: string): string {
  switch (lang) {
    case "Rust":
      return "#dea584";
    case "Nix":
      return "#7ebae4";
    case "TypeScript":
      return "#3178c6";
    case "JavaScript":
      return "#f1e05a";
    case "CSS":
      return "#563d7c";
    case "Go":
      return "#00ADD8";
    case "Makefile":
      return "#427819";
    case "Python":
      return "#3572A5";
    case "Jupyter Notebook":
      return "#DA5B0B";
    case "Shell":
      return "#89e051";
    case "Elm":
      return "#60B5CC";
    case "HTML":
      return "#e34c26";
    case "WGSL":
      return "#1e2b3a";
    case "Zig":
      return "#ec915c";
    case "Just":
      return "#384d54";
    case "Dockerfile":
      return "#384d54";
    case "Lua":
      return "#000080";
    case "GLSL":
      return "#5686a5";
    default:
      return "#000000";
  }
}

export default function ProjectCard({ project }: ProjectCardProps) {
  const strippedName = project.name?.replace(/"/g, "");
  const strippedDescription = project.description?.replace(/"/g, "");
  const strippedUrl = project.url?.replace(/"/g, "");
  const strippedHomepage = project.homepage?.replace(/"/g, "");

  const total_line_count = project.languages.reduce(
    (total, v) => total + v.line_count,
    0,
  );

  if (total_line_count === 0 || strippedDescription === "null") return null;

  const getPercentage = (lang: Language) =>
    (lang.line_count / total_line_count) * 100;

  return (
    <div className="border rounded-md p-4 flex flex-col gap-3 h-full">
      <div>
        <h3 className="text-lg font-semibold">{strippedName}</h3>
        {strippedDescription && strippedDescription !== "null" && (
          <p className="text-sm text-muted-foreground mt-2">
            {strippedDescription}
          </p>
        )}
      </div>

      <div className="flex flex-wrap gap-2 mt-auto pt-2">
        {project.languages
          .filter((lang) => getPercentage(lang) > 1)
          .map((lang) => {
            const percentage = getPercentage(lang).toFixed(1);
            return (
              <div key={lang.name} className="flex items-center space-x-1">
                <span
                  className="inline-block w-2 h-2 rounded-full"
                  style={{ backgroundColor: langColor(lang.name) }}
                ></span>
                <span className="text-xs">{lang.name}</span>
                <span className="text-xs text-muted-foreground">
                  {percentage}%
                </span>
              </div>
            );
          })}
      </div>

      <div className="flex items-center justify-between mt-auto pt-2 border-t">
        <div className="flex gap-2">
          {project.homepage != null && (
            <a target="_blank" href={strippedHomepage}>
              <span className="inline-flex items-center rounded-md px-2 py-0.5 text-xs font-medium bg-primary text-primary-foreground hover:bg-primary/90">
                Live
              </span>
            </a>
          )}
          <a target="_blank" href={strippedUrl}>
            <span className="inline-flex items-center rounded-md border px-2 py-0.5 text-xs font-medium text-foreground hover:bg-accent hover:text-accent-foreground">
              GitHub
            </span>
          </a>
        </div>

        {project.stars > 0 && (
          <div className="flex items-center text-xs">
            <span className="mr-1">⭐</span>
            <span>{project.stars}</span>
          </div>
        )}
      </div>
    </div>
  );
}
