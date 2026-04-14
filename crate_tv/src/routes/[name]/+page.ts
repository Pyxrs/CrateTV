import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, fetch }) => {
  const res = await fetch(`/api/stream/${params.name}`);
  const stream: { url: string } | null = res.ok ? await res.json() : null;

  return {
    info: {
      title: `${params.name}'s title`,
      description: `${params.name}'s description`,
    },
    stream_url: stream?.url ?? null,
  };
};
