<script>
    import { createEventDispatcher } from "svelte";
    import SettingsContainer from "../settings/SettingsContainer.svelte";
    import Description from "../settings/Description.svelte";
    import ButtonSetting from "../settings/ButtonSetting.svelte";

    export let launchVersionWarningCountdown;

    const dispatch = createEventDispatcher();
</script>

<SettingsContainer
        title="You are about to launch an unsupported version!"
        on:hideSettings={() => dispatch('hide')}
>
    <Description
            description="선택한 마인크래프트 버전은 구버전입니다. 게임 실행 시 권장 버전으로 업그레이드하거나 계속 진행할 수 있습니다."
    />
    <ButtonSetting
            text="Switch to Nextgen now"
            on:click={() => dispatch('switchToNextgen')}
            color="#4677FF"
    />
    <ButtonSetting
            disabled={launchVersionWarningCountdown > 0}
            text="Launch anyway{launchVersionWarningCountdown > 0 ? ` (${launchVersionWarningCountdown})` : ''}"
            on:click={() => dispatch('runClientAnyway')}
            color="#B83529"
    />
</SettingsContainer>