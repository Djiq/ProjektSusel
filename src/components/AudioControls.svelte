<svelte:options accessors/>
<script lang="ts">
    import { createEventDispatcher, onMount, tick } from "svelte";
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";

    export let song : Song;
    export let audio : HTMLAudioElement|null = null;
    export let paused = true;
    export let duration = 0;
    export let volume = 1;
    export let muted = false;
    export let preload = "metadata";

    export let shuffle = false;
    export let repeat = false;

    $: source = convertFileSrc(song.path);
    $: if(audio) audio.volume = volume;

    const dispatch = createEventDispatcher();

    let tooltipX = 0;
    let tooltipY = 0;
    let seekText = "";

    let seeking = false;
    let showTooltip = false;

    let timeline : HTMLProgressElement;
    let tooltip : HTMLDivElement;

    let currentTime = 0;

    export async function play()
    {
        audio?.play();
    }

    export async function pause()
    {
        audio?.pause();
    }

    export async function rewind()
    {
        dispatch("rewind");
        if(audio != null) 
            audio.currentTime = 0;
    }

    function seek(event : any, bounds : any) 
    {
        let x = event.pageX - bounds.left;
        return Math.min(Math.max(x / bounds.width, 0), 1);
    }

    function seekAudio(event: any) {
        if(!timeline) return;
        if(audio === null) return;
        audio.currentTime = seek(event, timeline.getBoundingClientRect()) * duration;
    }

    function seekTooltip(event: any) 
    {
        let tooltipBounds = tooltip.getBoundingClientRect();
        tooltipX = event.pageX - tooltipBounds.width - 10;
        tooltipY = timeline.offsetTop - 15;

        let bounds = timeline.getBoundingClientRect();
        let seekValue = (event.pageX - bounds.left) * duration / bounds.width;
        seekText = format(seekValue);
    }

    function trackMouse(event: any)
    {
        if(seeking) seekAudio(event);
        if(showTooltip) seekTooltip(event);
    }

    function format(seconds: any)
    {
        if(isNaN(seconds)) return "-:--";

        var time = parseInt(seconds, 10);
        var minute = Math.floor(time / 60);
        var second = Math.floor(time % 60);

        return [minute, second].map(v => v < 10 ? "0" + v : v).join(":");
    }
</script>

<svelte:window
    on:mouseup={() => seeking = false}
    on:mousemove={trackMouse}
/>

<div class="audio-controls">
    <div class="scrolling-marquee">
        {#if song.songid != 0}
            <p>
                Now playing: {song.name} {#if song.author} by {song.author} {/if}
            </p>
        {:else}
            <p>Add songs to queue to start playing</p>
        {/if}
    </div>
    <div class="audio-controls-buttons">
        <button class:active={shuffle} on:click={() => {dispatch("shuffle"); shuffle = !shuffle;}}>
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.484 9.166 15 7h5m0 0-3-3m3 3-3 3M4 17h4l1.577-2.253M4 7h4l7 10h5m0 0-3 3m3-3-3-3"/>
            </svg>
        </button>

        <button on:click={() => {rewind}}>
            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m17 16-4-4 4-4m-6 8-4-4 4-4"/>
              </svg>              
        </button>

        <button on:click={() => audio?.paused ? audio?.play() : audio?.pause()}>
            {#if audio?.paused}
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 18V6l8 6-8 6Z"/>
            </svg>
            {:else}
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M8 5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H8Zm7 0a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-1Z" clip-rule="evenodd"/>
            </svg>
            {/if}
        </button>

        <button on:click={() => dispatch("next")}>
            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m7 16 4-4-4-4m6 8 4-4-4-4"/>
            </svg>              
        </button>

        <button class:active={repeat} on:click={() => {dispatch("repeat"); repeat = !repeat;}}>
            <svg class="w-[48px] h-[48px] text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m16 10 3-3m0 0-3-3m3 3H5v3m3 4-3 3m0 0 3 3m-3-3h14v-3"/>
            </svg>
        </button>
    </div>
    <div class="progress-container">
        <span>
            {format(currentTime)}
        </span>
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <!-- svelte-ignore a11y-click-events-have-key-events -->
       <progress
            bind:this={timeline}
            value={currentTime ? currentTime : 0}
            max={duration}
            on:mousedown={() => seeking = true}
            on:mouseenter={() => showTooltip = true}
            on:mouseleave={() => showTooltip = false}
            on:click={seekAudio}> 
        </progress>
        <span>{format(duration)}</span>
    </div>
</div>

{#if showTooltip}
    <div class="tooltip hover-tooltip"
        transition:fade
        bind:this={tooltip}
        style="--left:{tooltipX}px;
            --top:{tooltipY}px;
            --background-color:white; --box-color:red;">
        {seekText}
    </div>
{/if}

{#if song.path != ""}
    <audio
        bind:this={audio}
        bind:paused
        bind:duration
        bind:currentTime
        {muted}
        on:play|preventDefault
        src={source}
        {preload}
        on:pause
        on:ended={() => {currentTime = 0; dispatch("ended");}}
    ></audio>
{/if}

<style>
.audio-controls {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.audio-controls-buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    margin: 7px 0 7px 0;
}

button {
    font-size: 1.5rem;
    border-radius: 99em;
    padding: 0;
    border: 0;
    display: grid;
    grid-template-columns: 1fr;
    place-items: center;
    cursor: pointer;
    width: 3.5rem;
    height: 2.5rem;
    transition: color 0.15s ease, width 0.25s ease-out;
    position: relative;
    background-color: var(--color-dp08);
    color:white;
    border: 1px solid var(--color-dp12);

    &:hover {
        background-color: var(--color-dp24);
        border: 1px solid var(--color-dp18);
    }
}

button.active {
    border: 2px solid var(--color-dp48);
}

.tooltip 
{
    background-color: white;
    padding: 1px;
    border-radius: 5px;
    border-width: 3px;
    color: red;
    pointer-events: none;
    min-width: 50px;
    text-align: center;
    margin-bottom: 5px;
}

.hover-tooltip
{
    position: absolute;
    top: var(--top);
    left: var(--left);
}

.progress-container 
{
    display: flex;
    align-items: baseline;
}

progress 
{
    min-width: 50vw;
    color: var(--color-dp24);
    background: var(--color-dp24);
    border: none;
    border-radius: 50%;
    height: 10px;
    margin: 0 0.3em 0 0.3em;
    border: 1px solid var(--color-dp24);
    border-radius: 15px;
}

.scrolling-marquee {
    white-space: nowrap;
    overflow: hidden;
    box-sizing: border-box;
    width: 100%;
}

.scrolling-marquee p {
    display: inline-block;
    padding-left: 100%;
    animation: marquee 10s linear infinite;
}

@keyframes marquee {
    0% { transform: translate(0, 0); }
    100% {transform: translate(-100%, 0);}
}

progress::-webkit-progress-bar {background-color: var(--color-dp04); width: 100%; border-radius: 15px;}

progress::-webkit-progress-value { background: color-mix(in srgb, #121212, white 75%);; border-radius: 15px; }
</style>