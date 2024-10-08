<script>
    import { invoke } from "@tauri-apps/api/core";
    export let path = '';
    export let name = 'unknown';

    async function selectSong(){
        await invoke("selectSong", { path,name });
    }
    
</script>

<button on:click={selectSong}>

</button>