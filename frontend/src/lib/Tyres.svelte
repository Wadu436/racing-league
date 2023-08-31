<script lang="ts">
	import type { TyreType$options } from '$houdini/graphql/enums';
	import iconHards from './assets/tires/hard.png';
	import iconMediums from './assets/tires/medium.png';
	import iconSofts from './assets/tires/soft.png';
	import iconInters from './assets/tires/inter.png';
	import iconWets from './assets/tires/wet.png';

	export let tyre: TyreType$options;

	let className = '';
	export { className as class };

	const icons: Record<TyreType$options, { url: string; name: string }> = {
		HARD: { url: iconHards, name: 'Hard tyres' },
		MEDIUM: { url: iconMediums, name: 'Medium tyres' },
		SOFT: { url: iconSofts, name: 'Soft tyres' },
		INTER: { url: iconInters, name: 'Intermediate tyres' },
		WET: { url: iconWets, name: 'Wet tyres' }
	};

	$: icon = icons[tyre];
</script>

<img src={icon.url} alt={icon.name} title={icon.name} class={className} />
