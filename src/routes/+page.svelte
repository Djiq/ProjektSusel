<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import EditableLabel from "../components/editableLabel.svelte";

    let addresses = []  //['10.10.10.10', '192.167.100.1', '8.8.8.8']
    let songs = []      //[{name: 'Test', path:'/mnt/test/test'}, {name: 'Test', path:'/mnt/test/test'}]

    async function updateServers()
    {
        //TODO: pass server list to backend
    }

    function addServer()
    {
        addresses.push('0.0.0.0')
        addresses = [...addresses]
    }
</script>

<div class="container">
    <div class="content">
        <p class='lato-regular'>File list</p>
        {#each songs as {name, path}}
            <div class='song-card'>
                <p>{name} {path}</p>
            </div>
        {/each}
    </div>

    <div class="top-bar">
    </div>

    <div class="side-bar">
        <p>Server List</p>

        {#each addresses as addr}
            <EditableLabel bind:value={addr} on:submit={updateServers}/>
        {/each}

        <button on:click={addServer}>Add server</button>
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