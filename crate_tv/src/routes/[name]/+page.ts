import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return {
    info: {
      title: `${params.name}'s title`,
      description: `${params.name}'s description`,
    },
  };
};
