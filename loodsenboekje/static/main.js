const app = {
    data() {
        return {
            'all_entries': [],
            'entries': [],
        };
    },
    created() {
        // Fetch data from the /entry endpoint
        fetch('/entry')
            .then(response => response.json())
            .then(data => {
                this.all_entries = data;
                this.entries = data;
            })
            .catch(error => {
                console.error('Error fetching data:', error);
            });
    },
    methods: {
        searching(_) {
            const fuse = new Fuse(this.all_entries, options);
            this.entries = this.searchText ? fuse.search(this.searchText).map(result => result.item) : this.all_entries;
        }
    },
};

// Setup Fuse
const options = {
  keys: ['description', 'collaborators']
}


// Mount the app
const vm = Vue.createApp(app).mount('#app');
