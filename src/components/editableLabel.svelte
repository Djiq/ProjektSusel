<script>
    import {createEventDispatcher, onMount } from 'svelte'

    export let value = '', required = true, placeholder=false;
    const dispatch = createEventDispatcher()
    let editable = false, value_prev = '';

    onMount(() => {
        value_prev = value
    })

    function edit() { editable = true }

    function update() 
    {
        if(value != value_prev)
        {
            dispatch('submit', value)
            value_prev = value
        }

        editable = false
    }

    function keydown(ev)
    {
        if(ev.key == 'Escape')
        {
            ev.preventDefault()
            value = value_prev
            editable = false
        }
    }

    function focus(element)
    {
        element.focus()
    }
</script>

{#if editable}
    <form on:submit|preventDefault={update} on:keydown={keydown}>
        <input class='label' bind:value on:blur={update} {required} use:focus/>
    </form>
{:else}
    <div on:click={edit} role='button'>
        <p class='label'>{value}</p>
    </div>
{/if}

<style>
.label {
    margin: 0.5em 0 0 0;
    border-radius: 0.3em;
    background-color: var(--base);
    min-height: 2em;
}
</style>