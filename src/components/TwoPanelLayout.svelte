<div class="content">
    <slot name="content"></slot>
</div>

<div class="side-bar">
    <slot name="side-bar"></slot>
</div>

<style>
.content { grid-area: content; }
.side-bar { grid-area: side-bar; }

.side-bar {
    background: var(--color-dp02);
    border-radius: var(--border-radius) 0 0 var(--border-radius);
    margin: 0 0 0.5em 1em;
    padding: 0.25em;
    overflow: auto;

    display: flex;
}

.content {
    background: var(--color-dp01);
    border-radius: 0 var(--border-radius) var(--border-radius) 0;
    margin: 0 1em 0.5em 0;

    display: flex;
}
</style>