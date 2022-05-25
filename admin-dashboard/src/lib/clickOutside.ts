export const clickOutside = (node, { enabled: initialEnabled, cb }) => {
	const handleOutsideClick = ({ target }) => {
		if (!node.contains(target)) {
			cb();
		}
	};

	const update = ({ enabled }) => {
		if (enabled) {
			window.addEventListener('click', handleOutsideClick);
		} else {
			window.removeEventListener('click', handleOutsideClick);
		}
	};

	update({ enabled: initialEnabled });
	return {
		update,
		destroy() {
			window.removeEventListener('click', handleOutsideClick);
		}
	};
};
