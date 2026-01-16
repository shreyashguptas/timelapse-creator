import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Timelapse Creator",
  description: "Create high-quality timelapse videos from image frames",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
