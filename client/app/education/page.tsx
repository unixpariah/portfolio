import EducationEntry from "./education-entry";

export default function EducationPage() {
  return (
    <>
      <div className="py-10">
        <h1 className="text-3xl font-bold">Education</h1>
        <h2 className="text-4xl font-bold text-muted-foreground">
          I learned a lot, but the real learning happens in the code editor!
        </h2>
      </div>

      <EducationEntry
        education={[
          {
            name: "WSB Merito",
            startDate: "2023",
            endDate: "Present",
            description:
              "Engaged in ongoing studies in Computer Science, focusing on coursework in programming languages, data structures, and software development methodologies.",
          },
          {
            name: "Zespół Szkół Elektrycznych Nr 2",
            startDate: "2019",
            endDate: "2023",
            description:
              "Completed a comprehensive IT Technician diploma program, acquiring expertise in troubleshooting hardware and software issues, network configuration, system maintenance practices, and web programming to enhance technical skills and knowledge.",
          },
        ]}
      />
    </>
  );
}
