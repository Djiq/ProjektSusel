<script lang="ts">
    import TwoPanelLayout from "src/components/TwoPanelLayout.svelte";

    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { parseWebStream } from "music-metadata";
    import { createEventDispatcher } from "svelte";

    export let current : Song;
    export let queue : Song[];
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

    function getSongDisplayData(song: Song, meta: any)
    {
        let result : any = {};
        result.title = meta?.common?.title ?? song.path.split('\\').pop()?.split('/').pop();
        song.name = result.title;

        result.artist = meta?.common?.artist ?? "Unknown artist";
        song.author = result.artist;

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
        <div><p class="content-header">Queue:</p></div>
        <div>
            {#each queue as song}
                <p>{song.name}</p>
            {/each}
        </div>
    </div>

    <div slot="content" class="song-library-container">
        <div><p class="content-header">Song library</p></div>

        <div class="song-card-container">
            {#await getAllSongs()}
                <p>Loading song list...</p>
            {:then songs}
                {#each songs as song}
                    {#await getMetadata(song)}
                        <button class="song-card">
                            <p class="song-title">Loading....</p>
                            <p class="song-author">...</p>
                        </button>
                    {:then meta}
                        {@const data = getSongDisplayData(song, meta)}
                        <button on:click={() => onSongSelect(song)} class="song-card">
                            <p class="song-title">{data.title}</p>
                            <p class="song-author">{data.artist} <span>{data.duration}</span></p>
                        </button>
                    {:catch}
                        {@const data = getSongDisplayData(song, null)}
                        <button on:click={() => onSongSelect(song)} class="song-card">
                            <p class="song-title">{data.title}</p>
                            <p class="song-author">{data.artist}</p>
                        </button>
                    {/await}
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
    width: 100%;
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
</style>