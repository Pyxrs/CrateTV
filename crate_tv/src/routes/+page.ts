import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return { followedStreamers, liveStreamers };
};

// Temp data
const followedStreamers = [
  {
    name: "TestUser1",
    avatar: "https://placehold.co/40",
    title: "Test 1",
    game: "Mario 64",
    preview: "https://placehold.co/320x180?text=TestUser1",
    viewers: 120,
  },
  {
    name: "TestUser2",
    avatar: "https://placehold.co/40",
    title: "Test 2",
    game: "Minecraft",
    preview: "https://placehold.co/320x180?text=TestUser2",
    viewers: 85,
  },
  {
    name: "TestUser3",
    avatar: "https://placehold.co/40",
    title: "Test 3",
    game: "Hollow Knight",
    preview: "https://placehold.co/320x180?text=TestUser3",
    viewers: 0,
  },
];

const liveStreamers = [
  {
    name: "TestUser1",
    avatar: "https://placehold.co/40",
    title: "Test 1",
    game: "Mario 64",
    preview: "https://placehold.co/320x180?text=TestUser1",
    viewers: 120,
  },
  {
    name: "TestUser2",
    avatar: "https://placehold.co/40",
    title: "Test 2",
    game: "Minecraft",
    preview: "https://placehold.co/320x180?text=TestUser2",
    viewers: 85,
  },
  {
    name: "TestUser4",
    avatar: "https://placehold.co/40",
    title: "Test 4",
    game: "Counter-Strike 2",
    preview: "https://placehold.co/320x180?text=TestUser4",
    viewers: 60,
  },
  {
    name: "TestUser7",
    avatar: "https://placehold.co/40",
    title: "Test 7",
    game: "Outer Wilds",
    preview: "https://placehold.co/320x180?text=TestUser7",
    viewers: 42,
  },
];
