import ExperienceEntry from "@/components/ui/experience-entry";

export default function ExperiencePage() {
  return (
    <>
      <div className="py-10">
        <h1 className="text-3xl font-bold">Experience</h1>
        <h2 className="text-4xl font-bold text-muted-foreground">
          You need it to get the job, but the job&rsquo;s what gives it!
        </h2>
      </div>

      <ExperienceEntry
        experiences={[
          {
            name: "Znak.pl",
            position: "Intern",
            startDate: "Jan&nbsp;2022",
            endDate: "2022",
            description:
              "Assisted in the assembly and configuration of custom PCs and laptops for clients, gaining hands‑on experience with various hardware components and system architectures. Conducted hardware diagnostics and troubleshooting for customer systems, resolving technical issues efficiently. Collaborated with senior technicians to implement proper cable management and optimize system performance, ensuring high‑quality builds that met client specifications.",
          },
          {
            name: "Komputronik",
            position: "Intern",
            startDate: "Dec&nbsp;2020",
            endDate: "Dec&nbsp;2020",
            description:
              "Independently assembled and configured custom PCs and laptops based on customer specifications and requirements. Installed and optimized operating systems, drivers, and essential software for newly built systems. Provided technical consultation to customers on hardware compatibility and optimal component selection based on their budget and performance needs. Maintained an organized workspace and inventory tracking, ensuring all components were accounted for during the assembly process.",
          },
        ]}
      />
    </>
  );
}
