import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        hostname: "avatars.githubusercontent.com",
      },
    ],
  },
  basePath: "/portfolio",
  assetPrefix: "/portfolio",
};

export default nextConfig;
