<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import EditableLabel from "../components/editableLabel.svelte";
    import { listen } from "@tauri-apps/api/event";

    let server = '';
    let files = [];
    
    function updateServerList()
    {
        invoke('ftplist', {servername: server})
            .then(file_list => files = file_list)
            .catch(e => files = ['Error: ' + e]);
    }

    listen('tauri://drag-drop', async (ev) => {
        for(const path of ev.payload.paths)
        {
            await invoke('addSong_invoc', {name:'test', path:path})
        }
    });
</script>

<div class="container">
    <div class="content">
        <p class='lato-regular'>File list</p>
        {#each files as file}
            <div class="song-card">
                <p>{file}</p>
            </div>
        {/each}
    </div>

    <div class="top-bar">
    </div>

    <div class="side-bar">
        <EditableLabel bind:value={server} on:submit={updateServerList}/>
    </div>
</div>

<style>
@import url('https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&family=Open+Sans:ital@0;1&display=swap');

* {
  font-family: "Lato", sans-serif;
  font-weight: 400;
  font-style: normal;
}

.content { grid-area: content; }
.top-bar { grid-area: top-bar; }
.side-bar { grid-area: side-bar; }

.side-bar {
    background: var(--shade2);
    border-radius: var(--border-radius);
    margin: 0 0 1em 1em;
    padding: 0.25em;
}

.content {
    background: var(--shade1);
    border-radius: var(--border-radius);
    margin: 0 1em 1em 1em;
}

.song-card 
{
    display: flex;
}

.top-bar {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    margin-left: 2em;
}
</style>