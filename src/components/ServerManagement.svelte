<script lang="ts">
    import TwoPanelLayout from "src/components/TwoPanelLayout.svelte";
    import { createEventDispatcher, onMount } from "svelte";

    export let serverList : any[];

    const dispatch = createEventDispatcher();

    let selected : any = null;

    let editor : any = {
        name: "",
        addr: ""
    }

    function selectServer(server : any)
    {
        editor.name = server.name
        editor.addr = server.ip

        selected = server;
    }

    function updateServer()
    {
        let old_name = selected.name;
        let old_ip = selected.ip;

        if(old_name != editor.name || old_ip != editor.addr)
        {
            selected.name = editor.name;
            selected.ip = editor.addr;

            serverList = serverList;
            dispatch("serverupdated", {old_name, selected});
        }
    }

    function setAdding()
    {
        selected = null;
        editor.name = ""
        editor.addr = ""
    }

    function addSever()
    {
        serverList.push({name: editor.name, ip: editor.addr});
        serverList = serverList;

        selectServer(serverList[serverList.length - 1])
        dispatch("serveradded", selected);
    }

    function deleteServer()
    {
        if(selected != null)
        {
            serverList = serverList.filter(x => x != selected);
            dispatch("serverremoved", selected.name);
            setAdding();
        }
    }
</script>

<TwoPanelLayout>
    <div slot="side-bar" class="side-bar">
        <div><p>Server List</p></div>

        <div class="server-card-container">
            {#each serverList as server}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-interactive-supports-focus -->
                <button on:click={() => selectServer(server)} class="server-card">
                    <p>{server.name}</p>
                    <p>{server.ip}</p>
                </button>
            {/each}
        </div>
        
        <button on:click={setAdding}>Add server</button>
    </div>

    <div slot="content" class="editor">
        <div class="server-editor">
            <div>
                <label for="server_name_in">Name</label>
                <input id="server_name_in" type="text" bind:value={editor.name}>
            </div>

            <div>
                <label for="server_addr_in">IP</label>
                <input id="server_addr_in" type="text" bind:value={editor.addr}>
            </div>

            {#if selected == null}
                <button on:click={addSever}>Add server</button>
            {:else}
                <div>
                    <button on:click={updateServer}>Save</button>
                    <button on:click={deleteServer}>Delete</button>
                </div>
            {/if}
        </div>
    </div>
</TwoPanelLayout>

<style>
label {
    display: inline-block;
    width: 5em;
    text-align: center;
}

button {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    padding: 0.3em;
    margin: 0.3em 1em 0.1em 1em;
    border: 0;
    cursor: pointer;
    background-color: var(--color-dp04);
    color:white;
    border: 1px solid var(--color-dp12);

    border-radius: var(--border-radius);

    &:hover {
        background-color: var(--color-dp16);
    }
}

.editor button {
    margin-top: 1em;
}

.editor {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex-grow: 1;
}

.side-bar {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
}

.server-card-container {
    overflow: auto; 
    height: 100%;
    display: flex;
    flex-direction: column;
}

.server-card {
    border: 1px solid var(--color-dp08);
    border-radius: var(--border-radius);
    padding: 0.4em;
    margin-bottom: 0.2em;

    &:hover {
        background-color: var(--color-dp08);
    }

    &:active {
        background-color: var(--color-dp24);
    }
}

.server-card p:nth-child(2) {
    color: var(--color-dp24);
}

.side-bar p {
    text-align: center;
}

.server-editor {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2em;

    background-color: var(--color-dp02);
    border: 1px solid var(--color-dp24);
}
</style>