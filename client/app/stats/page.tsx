export const dynamic = "force-dynamic";

import GitHubGraphs from "./GitHubGraphs";

const StatCard = ({
  title,
  value,
  className = "",
}: {
  title: string;
  value: string | number;
  className?: string;
}) => (
  <div
    className={`card border border-border/40 rounded-xl p-4 w-full h-full transition-transform duration-200 hover:scale-105 ${className}`}
  >
    <div className="card-content">
      <h3 className="text-lg font-semibold tracking-tight card-title text-muted-foreground">
        {title}
      </h3>
      <span className="text-5xl font-bold leading-tight tracking-tight card-value text-muted-foreground">
        {value}
      </span>
    </div>
  </div>
);

interface Stats {
  hireable: boolean;
  public_repos: number;
  followers: number;
  following: number;
  company: string;
  location: string;
}

export default async function Stats() {
  try {
    const res = await fetch("http://portfolio-server.portfolio/api/stats");
    if (!res.ok) throw new Error("Failed to fetch stats");

    const stats: Stats = await res.json();

    const githubStatCards = [
      {
        title: "Hireable",
        value: stats.hireable ? "Yes" : "No",
        className: stats.hireable ? "bg-green-500/20" : "",
      },
      {
        title: "Total Public Repositories",
        value: stats.public_repos || 0,
      },
      {
        title: "Followers",
        value: stats.followers || 0,
      },
      {
        title: "Following",
        value: stats.following || 0,
      },
      {
        title: "Current Company",
        value: stats.company || "N/A",
      },
      {
        title: "Location",
        value: stats.location || "N/A",
      },
    ];

    return (
      <>
        <div className="py-10">
          <h1 className="text-4xl font-bold">GitHub Stats</h1>
          <h2 className="text-4xl font-bold text-muted-foreground">
            Insights and metrics about my GitHub profile
          </h2>
        </div>

        <div className="flex items-center justify-center w-full p-4 mb-8 border border-border/40 rounded-xl">
          <GitHubGraphs />
        </div>

        <div className="mb-8">
          <div className="grid grid-cols-1 gap-4 card-container md:grid-cols-3">
            {githubStatCards.map((card, index) => (
              <StatCard
                key={index}
                title={card.title}
                value={card.value}
                className={card.className}
              />
            ))}
          </div>
        </div>
      </>
    );
  } catch (error) {
    console.error("Error fetching stats:", error);
    return (
      <div className="text-center text-red-500 mt-10">
        Failed to load GitHub stats.
      </div>
    );
  }
}
