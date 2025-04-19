"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import { FC } from "react";

export type NavItem = {
  title: string;
  href?: string;
  items?: NavItem[];
  label?: string;
  disabled?: boolean;
  external?: boolean;
};

type SidebarGroup = {
  title: string;
  items: NavItem[];
};

type SideNavProps = {
  config: {
    sidebarNav: SidebarGroup[];
  };
};

export const SideNav: FC<SideNavProps> = ({ config }) => {
  const pathname = usePathname();
  const items = config.sidebarNav;

  return items.length ? (
    <div className="flex flex-col gap-6">
      {items.map((group, index) => (
        <div key={index} className="flex flex-col gap-1">
          <h4 className="rounded-md px-2 py-1 text-sm font-medium">
            {group.title}
          </h4>
          {group.items?.length ? (
            <DocsNavItems items={group.items} pathname={pathname} />
          ) : null}
        </div>
      ))}
    </div>
  ) : null;
};

function DocsNavItems({
  items,
  pathname,
}: {
  items: NavItem[];
  pathname: string | null;
}) {
  return items?.length ? (
    <div className="grid grid-flow-row auto-rows-max gap-0.5 text-sm">
      {items.map((item) =>
        item.href && !item.disabled ? (
          <Link
            key={item.href}
            href={item.href}
            className={cn(
              "group relative flex h-8 w-full items-center rounded-lg px-2 after:absolute after:inset-x-0 after:inset-y-[-2px]  after:rounded-lg hover:bg-accent hover:text-accent-foreground ",
              item.disabled && "cursor-not-allowed opacity-60",
              pathname === item.href
                ? "bg-accent font-medium text-accent-foreground"
                : "font-normal text-foreground",
            )}
            target={item.external ? "_blank" : ""}
            rel={item.external ? "noreferrer" : ""}
          >
            {item.title}
            {item.label && (
              <span className="ml-2 rounded-md bg-[#adfa1d] px-1.5 py-0.5 text-xs leading-none text-[#000000] no-underline group-hover:no-underline">
                {item.label}
              </span>
            )}
          </Link>
        ) : (
          <span
            key={item.href ?? item.title}
            className={cn(
              "flex w-full cursor-not-allowed items-center rounded-md p-2 text-muted-foreground hover:underline",
              item.disabled && "cursor-not-allowed opacity-60",
            )}
          >
            {item.title}
            {item.label && (
              <span className="ml-2 rounded-md bg-muted px-1.5 py-0.5 text-xs leading-none text-muted-foreground no-underline group-hover:no-underline">
                {item.label}
              </span>
            )}
          </span>
        ),
      )}
    </div>
  ) : null;
}
