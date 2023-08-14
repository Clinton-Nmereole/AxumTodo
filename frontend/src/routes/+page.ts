import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	return {
		todos: await fetch('http://3.133.151.30:8000').then((data) => data.json())
	};
};
