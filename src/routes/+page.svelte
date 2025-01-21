<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { parseWebStream } from "music-metadata"
    import { tick } from "svelte";

    import AudioControls from "src/components/AudioControls.svelte";
    import ServerManagement from "src/components/ServerManagement.svelte";
    import NotImplemented from "src/components/NotImplemented.svelte";
    import AudioTracks from "src/components/AudioTracks.svelte";

    enum Tab {
        SONG = 0,
        PLAYLIST,
        SERVERS
    };

    let audio_controls : AudioControls;

    let song_current : Song = {
        path: "",
        name: "",
        album: "",
        songid: 0,
        author: null
    };

    let queueIndex = 0;
    let queue : Song[] = [];
    let state : Tab = Tab.SONG;

    async function changeTrack(song: Song)
    {
        audio_controls.pause();
        if(song_current.songid == song.songid)
            audio_controls.rewind();
        else
            song_current = song;

        await tick();
        audio_controls.play();
    }

    function enqueueSong(song: Song)
    {
        if(queue.length == 0 || queueIndex == queue.length)
        {
            changeTrack(song);
            queue = [...queue];
        }

        queue.push(song);
        queue = [...queue];
    }

    function shuffle_queue()
    {
        if(queueIndex >= queue.length - 1)
            return;

        let min = queueIndex;
        let max = queue.length;

        let rand = Math.floor(Math.random() * (max - min) + min);
        if(rand == queueIndex)
            return;

        [queue[queueIndex], queue[rand]] = [queue[rand], queue[queueIndex]];
    }

    function trackEnded()
    {
        queueIndex++;

        if(queueIndex == queue.length)
        {
            if(audio_controls?.repeat)
            {
                queueIndex = 0;
            }
            else
                return;
        }
        
        if(audio_controls?.shuffle)
            shuffle_queue();

        changeTrack(queue[queueIndex]);
        queue = [...queue];
    }

    function trackNext()
    {
        if(queue.length == 0)
            return;

        queueIndex++;
        if(queueIndex >= queue.length)
            queueIndex = 0;

        if(audio_controls?.shuffle)
            shuffle_queue();

        changeTrack(queue[queueIndex]);
        queue = [...queue];
    }

    async function server_added(event: CustomEvent)
    {
        await invoke("cmd_add_server", {name: event.detail.name, ip: event.detail.ip});
        let files : string[] = await invoke("cmd_ftplist", {servername: event.detail.ip});
        console.log(files);
        for(const file of files)
        {
            let path : string = await invoke("cmd_download", {servername: event.detail.ip, songname: file});
            let src = convertFileSrc(path);
            let metadata = await parseWebStream((await fetch(src)).body);

            let payload = {
                path: path,
                name: metadata?.common?.title ?? "",
                album: metadata?.common?.album ?? null,
                author: metadata?.common?.artist ?? null
            };

            console.log(path, metadata, payload);

            await invoke("cmd_add_song", payload);
        }
    }

    function queueCleared()
    {
        queue = [song_current];
        queueIndex = 0;
        audio_controls.pause();
    }

    function trackPlayed()
    {
        if(queueIndex >= queue.length)
        {
            queueIndex = 0;
            if(audio_controls?.shuffle)
                shuffle_queue();

            changeTrack(queue[queueIndex]);
        }
    }

    async function debugButton()
    {
        let test = convertFileSrc("D:\\Downloads\\Sunny day in peaceful town.mp3");
        const resp = await fetch(test);
        const stream = resp.body;

        parseWebStream(stream).then(x => console.log(x));

        // song_current.path = "https://sveltejs.github.io/assets/music/satie.mp3";
        // song_current.name = "test title";
        // await invoke("cmd_add_song", {path: "D:\\Downloads\\Eve_-_Headphone_Actor_(mp3.pm).mp3", name: "name_test", album: null, author: "me"});
    }
</script>

<div class="container">
    <button on:click={debugButton}> </button>
    <div class="top-bar">
        <button on:click={() => state = Tab.SONG} class="tab" class:active={state == Tab.SONG}>
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 18c0 1.1046-.89543 2-2 2s-2-.8954-2-2 .89543-2 2-2 2 .8954 2 2Zm0 0V6.33333L18 4v11.6667M8 10.3333 18 8m0 8c0 1.1046-.8954 2-2 2s-2-.8954-2-2 .8954-2 2-2 2 .8954 2 2Z"/>
              </svg>              
        </button>

        <button class="tab" on:click={() => state = Tab.PLAYLIST} class:active={state == Tab.PLAYLIST}>
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 15.5V5s3 1 3 4m-7-3H4m9 4H4m4 4H4m13 2.4c0 1.326-1.343 2.4-3 2.4s-3-1.075-3-2.4 1.343-2.4 3-2.4 3 1.075 3 2.4Z"/>
              </svg>              
        </button>

        <button class="tab" on:click={() => state = Tab.SERVERS} class:active={state == Tab.SERVERS}>
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M5 5a2 2 0 0 0-2 2v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V7a2 2 0 0 0-2-2H5Zm9 2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17ZM3 17v-3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Zm11-2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z" clip-rule="evenodd"/>
              </svg>              
        </button>
    </div>

    {#if state == Tab.SERVERS}
        <ServerManagement
            on:serveradded={server_added} 
            on:serverupdated={(s) => console.log(s.detail)}>
        </ServerManagement>
    {:else if state == Tab.SONG}
        <AudioTracks bind:queueIndex bind:queue on:select={(s) => enqueueSong(s.detail)} on:clear={queueCleared}></AudioTracks>
    {:else}
        <NotImplemented></NotImplemented>
    {/if}

    <div class="bottom-bar">
        <AudioControls 
            bind:this={audio_controls}
            bind:song={song_current}
            on:pause={() => console.log("pause")} 
            on:play={() => trackPlayed()}
            on:ended={() => trackEnded()}
            on:next={() => trackNext()}>
        </AudioControls>
    </div>
</div>

<style>
@import url('https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&family=Open+Sans:ital@0;1&display=swap');

* {
  font-family: "Lato", sans-serif;
  font-weight: 400;
  font-style: normal;
}

.top-bar { grid-area: top-bar; }
.bottom-bar {grid-area: bottom-bar; }

.top-bar {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    margin-left: 2em;
}

.tab {
    padding: 0.5em;
    background-color: var(--color-dp00);
    border: none;
    color: white;
    margin-right: 1em;
    font-size: large;

    &:hover {
        background-color: var(--color-dp04);
    }
}

.active {
    background-color: var(--color-dp01);
}

.bottom-bar {
    display: flex;
    background: var(--color-dp01);
    border-radius: var(--border-radius);
    margin: 0 1em 1em 1em;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>