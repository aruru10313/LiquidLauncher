<script>
    import SocialBar from "../../common/social/SocialBar.svelte";
</script>

<div class="news-area">
    <div class="info-card">
        <h3>xmasLegacy Launcher</h3>
        <p>xmasLegacy 런처에 오신 것을 환영합니다! 안전하고 쾌적한 마인크래프트 플레이 환경을 제공합니다.</p>
    </div>

    <div class="social-bar-wrapper">
        <SocialBar />
    </div>
</div>

<style>
    .news-area {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        overflow: hidden;
    }

    .info-card {
        background-color: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        padding: 24px;
        color: #ffffff;
    }

    .info-card h3 {
        margin-top: 0;
        margin-bottom: 12px;
        font-size: 20px;
        color: #4677FF;
    }

    .info-card p {
        margin: 0;
        line-height: 1.5;
        color: rgba(255, 255, 255, 0.7);
    }

    .social-bar-wrapper {
        display: flex;
        justify-content: flex-end;
    }
</style>

