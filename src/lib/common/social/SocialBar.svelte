<script>
    import ButtonIcon from "./ButtonIcon.svelte";
    import ButtonIconText from "./ButtonIconText.svelte";
</script>

<div class="social-bar">
    <ButtonIcon icon="github" text="GitHub" url="https://github.com/aruru10313/LiquidLauncher"/>
</div>

<style>
    .social-bar {
        background-color: rgba(0, 0, 0, .26);
        border-radius: 6px;
        display: flex;
        width: max-content;
        padding: 0 20px;
        padding: 8px;
        column-gap: 15px;
        align-items: center;
    }
</style>
