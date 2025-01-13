document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('a').forEach(link => {
    // Check if the link is external
    if (link.href && link.host !== window.location.host) {
      link.setAttribute('target', '_blank');
      link.setAttribute('rel', 'noopener noreferrer'); // Security best practice
    }
  });
});

