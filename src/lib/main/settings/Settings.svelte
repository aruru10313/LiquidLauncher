<script>
    import {createEventDispatcher} from "svelte";
    import GeneralSettings from "./GeneralSettings.svelte";
    import AboutSettings from "./AboutSettings.svelte";
    import SettingsContainer from "../../settings/SettingsContainer.svelte";
    import Tabs from "../../settings/tab/Tabs.svelte";


    export let options;
    let activeSettingsTab = "General";

    const dispatch = createEventDispatcher();
</script>

<SettingsContainer
        title="Settings"
        on:hideSettings={() => dispatch('hide')}
>
    <Tabs
            tabs={["General", "About"]}
            bind:activeTab={activeSettingsTab}
            slot="tabs"
    />

    {#if activeSettingsTab === "General"}
        <GeneralSettings
                bind:options
        />
    {:else if activeSettingsTab === "About"}
        <AboutSettings />
    {/if}
</SettingsContainer>