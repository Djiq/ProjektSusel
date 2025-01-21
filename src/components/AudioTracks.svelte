<script lang="ts">
    import TwoPanelLayout from "src/components/TwoPanelLayout.svelte";

    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { parseWebStream } from "music-metadata";
    import { createEventDispatcher } from "svelte";

    export let queue : Song[];
    export let queueIndex: number;

    const dispatch = createEventDispatcher();

    async function getAllSongs() : Promise<Song[]>
    {
        return await invoke("cmd_get_all_songs");
    }

    async function getMetadata(song: Song)
    {
        let path = convertFileSrc(song.path);
        return await parseWebStream((await fetch(path)).body);
    }

    async function getSongDisplayData(song: Song)
    {
        let result : any = {};
        result.title = song.name != "" ? song.name : song.path.split('\\').pop()?.split('/').pop();
        result.artist = song.author ?? "Unknown artist";

        let src = convertFileSrc(song.path);
        let meta = await parseWebStream((await fetch(src)).body);

        let time = meta?.format?.duration ?? 0;
        let minute = Math.floor(time / 60);
        let second = Math.floor(time % 60);
        result.duration = [minute, second].map(v => v < 10 ? "0" + v : v).join(":");

        return result;
    }

    function onSongSelect(song: Song)
    {
        dispatch("select", song);
    }
</script>

<TwoPanelLayout>
    <div slot="side-bar" class="queue-container">
        <div class="header-container-queue">
            <p class="content-header">Queue</p>
            <button class="clear-button" on:click={() => dispatch("clear")}>
                <svg class="w-[12px] h-[12px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 7h14m-9 3v8m4-8v8M10 3h4a1 1 0 0 1 1 1v3H9V4a1 1 0 0 1 1-1ZM6 7h12v13a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V7Z"/>
                  </svg>
            </button>
            </div>
        <div class="queue-card-holder">
            <div>
            {#each queue as song, index}
                <div class="queue-card" class:current={index == queueIndex}>
                    <p>{song.name}</p>
                </div>
            {:else}
                <p class="gray-text">No songs queued...</p>
            {/each}
            </div>
        </div>
    </div>

    <div slot="content" class="song-library-container">
        <div><p class="content-header">Song library</p></div>

        <div class="song-card-container">
            {#await getAllSongs()}
                <p>Loading song list...</p>
            {:then songs}
                {#each songs as song}
                    {#await getSongDisplayData(song)}
                    <button class="song-card">
                        <p class="song-title">Loading...</p>
                        <p class="song-author">...<span></span></p>
                    </button>
                    {:then data}
                    <button on:click={() => onSongSelect(song)} class="song-card">
                        <p class="song-title">{data.title}</p>
                        <p class="song-author">{data.artist} <span>{data.duration}</span></p>
                    </button>
                    {/await}
                {:else}
                    <div class="tutorial-box">
                        <p>No songs found; Add servers to download songs via <span><svg class="w-[12px] h-[12px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M5 5a2 2 0 0 0-2 2v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V7a2 2 0 0 0-2-2H5Zm9 2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17ZM3 17v-3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Zm11-2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z" clip-rule="evenodd"/></svg> tab</span></p>
                    </div>
                {/each}
            {:catch err}
                <p>Error: {err}</p>
            {/await}
        </div>
    </div>
</TwoPanelLayout>

<style>
.song-library-container {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
}

.song-card-container {
    overflow: auto; 
    height: 100%;
    display: flex;
    flex-direction: column;
}

.queue-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-grow: 1;
    overflow: hidden;
    white-space: nowrap;
}

button {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    text-align: left;

    padding: 0.3em;
    margin: 0 1em 0 1em;
    border: 0;
    cursor: pointer;
    background-color: var(--color-dp04);
    color:white;

    &:hover {
        background-color: var(--color-dp16);
    }
}

.tutorial-box {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    flex-direction: row;
    align-items: center;
    color: var(--color-dp48);
}


.tutorial-box p {
    margin: 2em;
}

.content-header {
    font-size: large;
    text-align: center;
    margin: 0.5em 0;
}


.song-title {
    font-weight: bold;
}

.song-author {
    font-size: small;
    color: var(--color-dp65);
}
.song-author span {
    color: var(--color-dp48);
}

.song-card {
    background: var(--color-dp16);
    padding: 0.25em 0 0.25em 1em;

    &:hover {
        background-color: var(--color-dp04);
    }

    &:active {
        background-color: var(--color-dp24);
    }
}

.song-card:nth-child(2n) {
    background-color: var(--color-dp08);

    &:hover {
        background-color: var(--color-dp04);
    }
}

.gray-text {
    color: var(--color-dp16);
    text-align: center;
}

.clear-button {
    margin: 0;
    padding: 0;
    background: none;
    height: 1ch;
}

.header-container-queue {
    display: flex;
    align-items: flex-start;
    justify-content: space-evenly;
    padding: 0.3em 0 0.3em 0;
}

.header-container-queue p {
    margin: 0;
}

.queue-card {
    margin: 0.3em;
    padding: 0.6em;
    background-color: var(--color-dp08);
    text-align: center;
}

.queue-card p {
    display: block;
    text-overflow: ellipsis;
    overflow: hidden;
}

.queue-card:hover {
    white-space: nowrap;
    overflow: hidden;
    box-sizing: border-box;
    font-size: large;

    background-color: var(--color-dp16);
}

.queue-card:hover p, .queue-card.current p {
    display: inline-block;
    padding-left: 100%;
    animation: marquee 8s linear infinite;
}

.queue-card.current {
    border: 1px solid var(--color-dp48);
    white-space: nowrap;
    overflow: hidden;
    box-sizing: border-box;
}

@keyframes marquee {
    0% { transform: translate(0, 0); }
    100% {transform: translate(-100%, 0);}
}

.queue-card-holder {
    max-height: 100%;
    overflow-y: auto;
}
</style>