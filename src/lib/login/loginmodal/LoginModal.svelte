<script>
    import ModalButton from "./ModalButton.svelte";
    import ModalInput from "./ModalInput.svelte";

    import {invoke} from "@tauri-apps/api/core";
    import {listen} from "@tauri-apps/api/event";
    import {openUrl} from "@tauri-apps/plugin-opener";

    export let options;

    let microsoftCode = null;

    async function handleMicrosoftLoginClick(e) {
        try {
            options.start.account = await invoke("login_microsoft");
            options.store();
        } catch (err) {
            alert(
                "Microsoft authentication failed.\n\n" +
                 err
            );
            cancelMicrosoft();
        }
    }

    listen("microsoft_code", (e) => {
        microsoftCode = e.payload;
    });

    function cancelMicrosoft() {
        microsoftCode = null;
    }
</script>

<div class="modal">
    {#if !microsoftCode}
        <div class="title">Log in</div>


        <ModalButton text="Microsoft login" primary={true} on:click={handleMicrosoftLoginClick} />
    {:else}
        <div class="title">Microsoft Login</div>

        <ModalInput placeholder="Microsoft Code" characterLimit={16} icon="lock" bind:value={microsoftCode} />
        <ModalButton text="Link" primary={true} on:click={() => openUrl("https://microsoft.com/link")} />
        <ModalButton text="Cancel" primary={false} on:click={cancelMicrosoft} />
    {/if}
</div>

<style>
    .modal {
        background-color: rgba(0, 0, 0, 0.26);
        padding: 30px;
        border-radius: 12px;
        width: 320px;
        display: flex;
        flex-direction: column;
        row-gap: 15px;
    }

    .title {
        color: white;
        font-size: 22px;
        margin: 0 auto;
        position: relative;
        width: max-content;
        margin-bottom: 40px;
    }

    .title::after {
        content: "";
        position: absolute;
        height: 5px;
        width: calc(100% - 10px);
        left: 50%;
        bottom: -20px;
        transform: translateX(-50%);
        background-color: #4677FF;
        border-radius: 5px;
    }
</style>