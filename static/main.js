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
                credentials: 'same-origin',
                body: JSON.stringify({
                    'how': this.how,
                    'who': this.who,
                }),
            // Add data right away
            }).then(response => response.json()).then(data => {
                this.all_entries.push({
                    'id': data.id,
                    'how': data.how,
                    'who': data.who,
                    'created': data.created,
                });
            })
            // Reset after submitting
            this.open = false;
            this.how = "";
            this.who = "";
        },
        deleteEntry(entry_id) {
            fetch(`/entry/${entry_id}`, {
                method: 'DELETE',
            });
            this.all_entries = this.all_entries.filter(entry => entry.id !== entry_id);
            this.entries = this.entries.filter(entry => entry.id !== entry_id);
        },
        editEntry(e) {
            e.editing = true;
        },
        cancelEdit(e) {
            e.editing = false;
        },
        saveEntry(e) {
            e.editing = false;
            fetch(`/entry/${e.id}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'same-origin',
                body: JSON.stringify({
                    'how': e.how,
                    'who': e.who,
                    'created': e.created,
                }),
            }).then(response => response.json()).then(data => {
                e.created = data.created;
            })
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
