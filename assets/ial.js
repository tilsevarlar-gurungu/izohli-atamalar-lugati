document.addEventListener("DOMContentLoaded", function() {
    const abbrevations = document.querySelectorAll('abbr[data-en-ru]');

    abbrevations.forEach(el => {
        // Create box if missing
        if (!el.querySelector('.tooltip-box')) {
            const hint = el.getAttribute('data-en-ru');
            const box = document.createElement('span');
            box.className = 'tooltip-box';
            box.textContent = hint;
            el.appendChild(box);
        }

        const box = el.querySelector('.tooltip-box');

        function adjustPosition() {
            // 1. Get Measurements
            const parentRect = el.getBoundingClientRect();
            const boxRect = box.getBoundingClientRect();
            const viewportWidth = document.documentElement.clientWidth;
            const padding = 15; // Safety gap from screen edge

            // 2. Calculate "Ideal" Global Center
            // (Parent Left + Half Parent Width) - (Half Tooltip Width)
            let idealLeft = parentRect.left + (parentRect.width / 2) - (boxRect.width / 2);

            // 3. Clamp to Viewport Boundaries
            // Cannot be less than padding (Left Edge)
            if (idealLeft < padding) {
                idealLeft = padding;
            }
            // Cannot be more than (Screen Width - Tooltip Width - Padding) (Right Edge)
            else if ((idealLeft + boxRect.width) > (viewportWidth - padding)) {
                idealLeft = viewportWidth - boxRect.width - padding;
            }

            // 4. Convert Global Coordinate back to Relative (because position is absolute)
            // The box's 'left: 0' aligns with parentRect.left.
            // So we subtract parentRect.left from our target Global Left.
            const finalRelativeLeft = idealLeft - parentRect.left;

            // 5. Apply
            box.style.left = `${finalRelativeLeft}px`;
        }

        // Trigger on Hover/Touch
        el.addEventListener('mouseenter', adjustPosition);
        el.addEventListener('touchstart', adjustPosition, { passive: true });

        el.addEventListener('click', (e) => {
            e.stopPropagation();
            const isActive = el.classList.contains('active');

            document.querySelectorAll('abbr.active').forEach(a => a.classList.remove('active'));

            if (!isActive) {
                // Must calculate BEFORE adding active class so it pops up in right place
                adjustPosition();
                el.classList.add('active');
            }
        });
    });

    // Close on global click
    document.addEventListener('click', () => {
        document.querySelectorAll('abbr.active').forEach(a => a.classList.remove('active'));
    });
});
