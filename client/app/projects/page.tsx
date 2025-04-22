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

const res = await fetch("http://localhost:8000/projects");
const projectsData: Project[] = await res.json();

export default function ProjectsPage() {
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
}
