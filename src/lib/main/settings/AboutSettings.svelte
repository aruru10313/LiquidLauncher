<script>
    import Description from "../../settings/Description.svelte";
    import ButtonSetting from "../../settings/ButtonSetting.svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";
</script>

<div class="about-container">
    <Description
        description="xmasLegacy Launcher는 마인크래프트 실행을 위한 깔끔하고 가벼운 커스텀 런처입니다."
    />

    <Description
        description="본 프로젝트는 CCBlueX(Marco Beyer)의 LiquidLauncher 오픈소스 프로젝트를 기반으로 제작되었습니다. (GNU General Public License v3.0)"
    />

    <ButtonSetting
        text="GitHub Repository"
        on:click={() => openUrl("https://github.com/aruru10313/LiquidLauncher")}
        color="#4677FF"
    />
</div>

<style>
    .about-container {
        display: flex;
        flex-direction: column;
        row-gap: 12px;
    }
</style>
