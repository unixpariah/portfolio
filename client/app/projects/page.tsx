export const dynamic = "force-dynamic";

import ProjectCard from "@/components/ui/project-card";

export interface Language {
  name: string;
  line_count: number;
}

export interface Project {
  id: string;
  name: string;
  description: string;
  stars: number;
  url: string;
  homepage: string | null;
  languages: Language[];
}

export default async function ProjectsPage() {
  try {
    const res = await fetch("http://server:8000/projects");
    if (!res.ok) throw new Error("Failed to fetch projects");

    const projectsData: Project[] = await res.json();

    return (
      <div className="py-10">
        <h1 className="text-3xl font-bold mb-6">Projects</h1>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {projectsData.map((project) => (
            <ProjectCard key={project.id} project={project} />
          ))}
        </div>
      </div>
    );
  } catch (error) {
    console.log("Error fetching projects:", error);
    return (
      <div className="text-center text-red-500 mt-10">
        Failed to load projects.
      </div>
    );
  }
}
