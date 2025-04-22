"use client";

import { useEffect, useState } from "react";
import GitHubCalendar from "react-github-calendar";
import { useTheme } from "next-themes";

export default function GitHubGraphs() {
  const { resolvedTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  const theme = {
    dark: ["#161b22", "#0e4429", "#006d32", "#26a641", "#39d353"],
    light: ["#ebedf0", "#9be9a8", "#40c463", "#30a14e", "#216e39"],
  };

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return null;
  }

  const getThreeMonthsAgo = () => {
    const date = new Date();
    date.setMonth(date.getMonth() - 3);
    return date;
  };

  const transformThreeMonths = (data: any[]) => {
    const threeMonthsAgo = getThreeMonthsAgo();
    return data.filter((activity) => new Date(activity.date) >= threeMonthsAgo);
  };

  return (
    <>
      {/* Mobile view (last 3 months) */}
      <div className="md:hidden w-full">
        <GitHubCalendar
          username="unixpariah"
          errorMessage="Could not fetch GitHub stats"
          hideColorLegend
          theme={theme}
          colorScheme={resolvedTheme as "light" | "dark"}
          showWeekdayLabels={true}
          blockMargin={6}
          transformData={transformThreeMonths}
        />
      </div>

      {/* Desktop view (full calendar) */}
      <div className="hidden md:block w-full">
        <GitHubCalendar
          username="unixpariah"
          errorMessage="Could not fetch GitHub stats"
          hideColorLegend={false}
          theme={theme}
          colorScheme={resolvedTheme as "light" | "dark"}
          showWeekdayLabels={true}
          blockMargin={6}
        />
      </div>
    </>
  );
}
