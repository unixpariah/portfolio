"use client";

import { useEffect, useState } from "react";
import GitHubCalendar, {
  Activity,
  Props as GitHubCalendarProps,
} from "react-github-calendar";
import { useTheme } from "next-themes";

export default function GitHubGraphs() {
  const { resolvedTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  const theme: NonNullable<GitHubCalendarProps["theme"]> = {
    dark: ["#161b22", "#0e4429", "#006d32", "#26a641", "#39d353"],
    light: ["#ebedf0", "#9be9a8", "#40c463", "#30a14e", "#216e39"],
  };

  useEffect(() => setMounted(true), []);
  if (!mounted) return null;

  const threeMonthsAgo = (() => {
    const d = new Date();
    d.setMonth(d.getMonth() - 3);
    return d;
  })();

  const transformThreeMonths = (data: Activity[]): Activity[] =>
    data.filter((d) => new Date(d.date) >= threeMonthsAgo);

  return (
    <>
      {/* Mobile view (last 3 months) */}
      <div className="w-full md:hidden">
        <GitHubCalendar
          username="unixpariah"
          errorMessage="Could not fetch GitHub stats"
          hideColorLegend
          theme={theme}
          colorScheme={resolvedTheme as "light" | "dark"}
          showWeekdayLabels
          blockMargin={6}
          transformData={transformThreeMonths}
        />
      </div>

      {/* Desktop view (full calendar) */}
      <div className="hidden w-full md:block">
        <GitHubCalendar
          username="unixpariah"
          errorMessage="Could not fetch GitHub stats"
          hideColorLegend={false}
          theme={theme}
          colorScheme={resolvedTheme as "light" | "dark"}
          showWeekdayLabels
          blockMargin={6}
        />
      </div>
    </>
  );
}
