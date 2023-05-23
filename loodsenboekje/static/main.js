const app = {
    data() {
        return {
            'all_entries': [],
            'entries': [],
            'open': false,
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
        },
        addEntry(e) {
            e.preventDefault();
            this.how = this.how.trim();
            this.who = this.who.trim().split(',');
            // Post data to the /entry endpoint
            fetch('/entry', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    'how': this.how,
                    'who': this.who,
                }),
            })
            // Add locally for instant update, TODO: also add id and timestamp?
            this.all_entries.push({
                'how': this.how,
                'who': this.who,
            });
            // Reset after submitting
            this.open = false;
            this.how = "";
            this.who = "";
        },
        handleToggle(e) {
            this.open = e.target.open;
        }
    },
};

// Setup Fuse
const options = {
    keys: ['how', "who"]
}

// Mount the app
const vm = Vue.createApp(app).mount('#app');
