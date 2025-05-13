<script lang="ts">
  import logo from "$lib/images/logo.svg";
  import account from "$lib/images/account.svg";
  import { goto } from "$app/navigation";
  let searchTerm = "";

  function handleSearch(event: Event) {
    event.preventDefault();
    goto(`/${encodeURIComponent(searchTerm)}`);
  }
</script>

<header>
  <div class="corner left">
    <a href="/" aria-label="Home">
      <img src={logo} alt="Home" />
    </a>
  </div>

  <form
    class="search-container"
    on:submit|preventDefault={handleSearch}
    role="search"
    aria-label="Site search"
  >
    <input
      type="text"
      placeholder="Search..."
      bind:value={searchTerm}
      aria-label="Search"
    />
    <button type="submit" aria-label="Search">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        width="18"
        height="18"
      >
        <path
          fill="currentColor"
          d="M15.5 14h-.79l-.28-.27a6.5 6.5 0 1 0-.7.7l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0A4.5 4.5 0 1 1 14 9.5 4.5 4.5 0 0 1 9.5 14z"
        />
      </svg>
    </button>
  </form>

  <div class="corner right">
    <a href="/account" aria-label="Account">
      <img src={account} alt="Account" />
    </a>
  </div>
</header>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100vw;
    min-width: 320px;
    background: rgba(44, 46, 51, 0.98);
    backdrop-filter: blur(10px);
    padding: 0.2rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    position: relative;
    box-sizing: border-box;
  }

  .corner {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3.5em;
    height: 3.5em;
    padding: 0;
  }

  .corner.left {
    margin-right: auto;
  }

  .corner.right {
    margin-left: auto;
  }

  .corner a {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    transition: transform 0.2s ease;
  }

  .corner a:hover {
    transform: scale(1.1);
  }

  .corner img {
    width: 2.2em;
    height: 2.2em;
    object-fit: contain;
    display: block;
  }

  .search-container {
    flex: 1 1 0;
    max-width: 600px;
    position: relative;
    margin: 0 2rem;
    display: flex;
    align-items: center;
  }

  input[type="text"] {
    width: 100%;
    height: 38px;
    padding: 0 44px 0 16px;
    border-radius: 8px;
    border: none;
    background: rgba(255, 255, 255, 0.1);
    color: var(--color-text-1, #222);
    font-size: 1rem;
    box-sizing: border-box;
  }

  input[type="text"]:focus {
    outline: none;
    background: rgba(255, 255, 255, 0.15);
  }

  button[type="submit"] {
    position: absolute;
    right: 4px;
    top: 50%;
    transform: translateY(-50%);
    height: 30px;
    width: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--color-text-1, #222);
    cursor: pointer;
    opacity: 0.8;
    padding: 0;
  }

  button[type="submit"]:hover {
    opacity: 1;
  }
</style>
