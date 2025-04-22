"use client";

import type React from "react";

import { cn } from "@/lib/utils";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Briefcase,
  Code,
  Contact,
  GraduationCap,
  Home,
  BarChart,
  PenToolIcon as Tool,
  User,
} from "lucide-react";

interface NavItem {
  title: string;
  href: string;
  icon: React.ReactNode;
}

const navItems: NavItem[] = [
  {
    title: "Introduction",
    href: "/",
    icon: <Home className="h-4 w-4" />,
  },
  {
    title: "About Me",
    href: "/about",
    icon: <User className="h-4 w-4" />,
  },
  {
    title: "Projects",
    href: "/projects",
    icon: <Code className="h-4 w-4" />,
  },
  {
    title: "Skills & Tools",
    href: "/skills",
    icon: <Tool className="h-4 w-4" />,
  },
  {
    title: "Experience",
    href: "/experience",
    icon: <Briefcase className="h-4 w-4" />,
  },
  {
    title: "Education",
    href: "/education",
    icon: <GraduationCap className="h-4 w-4" />,
  },
  {
    title: "Contact",
    href: "/contact",
    icon: <Contact className="h-4 w-4" />,
  },
  {
    title: "Stats",
    href: "/stats",
    icon: <BarChart className="h-4 w-4" />,
  },
];

interface LeftSidebarProps {
  open?: boolean;
}

export default function LeftSidebar({ open = true }: LeftSidebarProps) {
  const pathname = usePathname();

  return (
    <aside
      className={cn(
        "fixed inset-y-0 left-0 z-30 mt-16 w-64 transform border-r bg-background transition-transform duration-200 ease-in-out md:static md:translate-x-0",
        open ? "translate-x-0" : "-translate-x-full",
      )}
    >
      <div className="flex h-full flex-col overflow-y-auto py-4">
        <nav className="space-y-1 px-2">
          {navItems.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                "flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground",
                pathname === item.href
                  ? "bg-accent text-accent-foreground"
                  : "text-muted-foreground",
              )}
            >
              {item.icon}
              <span className="ml-3">{item.title}</span>
            </Link>
          ))}
        </nav>
      </div>
    </aside>
  );
}
