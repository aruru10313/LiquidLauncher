<script>
    import {createEventDispatcher} from "svelte";
    import SettingsContainer from "../settings/SettingsContainer.svelte";
    import Description from "../settings/Description.svelte";
    import ButtonSetting from "../settings/ButtonSetting.svelte";
    import {openUrl} from "@tauri-apps/plugin-opener";

    const dispatch = createEventDispatcher();
</script>

<SettingsContainer
        title="xmasLegacy Launcher에 오신 것을 환영합니다!"
        on:hideSettings={() => dispatch('hide')}
>
    <Description
            description="xmasLegacy Launcher를 이용해 주셔서 감사합니다. 원활한 게임 실행을 위해 아래 진행 버튼을 눌러주세요."
    />

    <ButtonSetting
            text="진행하기"
            on:click={() => dispatch('continue')}
            color="#4677FF"
    />
</SettingsContainer>