# JDeduplix: AI-Powered Smart Deduplication System

JDeduplix is a cutting-edge deduplication system that leverages artificial intelligence to provide intelligent, accurate, and efficient data deduplication across multiple data types.

## 📊 System Architecture

### Full Stack Overview

```mermaid
graph TD
    classDef frontend fill:#42b883,stroke:#35495e,stroke-width:2px,color:white
    classDef backend fill:#2b7489,stroke:#1a1c1d,stroke-width:2px,color:white
    classDef storage fill:#ff6b6b,stroke:#c92a2a,stroke-width:2px,color:white
    
    subgraph "Frontend Layer"
        UI[Tauri + Vue UI]:::frontend
        VIS[Visualizations]:::frontend
        EXP[Export Tools]:::frontend
    end

    subgraph "Backend Layer"
        DE[Deduplication Engine]:::backend
        AI[AI Processing]:::backend
        IDX[Smart Indexing]:::backend
    end

    subgraph "Storage Layer"
        DB[(Databases)]:::storage
        FS[(File System)]:::storage
        VDB[(Vector DBs)]:::storage
    end

    UI --> DE
    DE --> AI
    AI --> IDX
    IDX --> DB & FS & VDB
    IDX --> UI
    UI --> VIS & EXP

    style Frontend fill:#e8f5e9,stroke:#81c784,stroke-width:2px
    style Backend fill:#e3f2fd,stroke:#64b5f6,stroke-width:2px
    style Storage fill:#ffebee,stroke:#ef9a9a,stroke-width:2px
```

### Frontend Architecture

```mermaid
flowchart TD
    classDef primary fill:#42b883,stroke:#35495e,stroke-width:2px,color:white
    classDef secondary fill:#3eaf7c,stroke:#2c3e50,stroke-width:2px,color:white
    classDef action fill:#4fc08d,stroke:#2c3e50,stroke-width:2px,color:white

    subgraph UI[User Interface]
        direction TB
        A[User Dashboard]:::primary
        B[Upload Interface]:::primary
        C[Results View]:::primary
        D[Smart Classifier UI]:::secondary
        E[Manual Resolution]:::secondary
        F[AI Confidence Display]:::secondary
        G[Similarity Heatmap]:::secondary
        H[Export Options]:::action
        I[Download]:::action
        J[Share]:::action
        
        A --> B
        A --> C
        B --> D
        C --> E
        C --> F
        C --> G
        E & F & G --> H
        H --> I
        H --> J
    end

    style UI fill:#f8f9fa,stroke:#42b883,stroke-width:2px
```

### Backend Architecture

```mermaid
flowchart TD
    classDef engine fill:#2b7489,stroke:#1a1c1d,stroke-width:2px,color:white
    classDef ai fill:#6b9fff,stroke:#2d5a9e,stroke-width:2px,color:white
    classDef storage fill:#ff6b6b,stroke:#c92a2a,stroke-width:2px,color:white

    subgraph Core[Deduplication Core]
        direction TB
        A[Smart Classifier]:::engine
        B[Deduplication Engine]:::engine
        C[AI Processing Pipeline]:::ai
        D1[ML Text Deduper]:::ai
        D2[GNN JSON Deduper]:::ai
        D3[CNN Image Deduper]:::ai
        D4[Binary Deduper]:::ai
        E[ML Conflict Resolver]:::ai
        F[Auto-Tuning]:::ai
        G[Vector Indexing]:::engine
        H1[FAISS HNSW]:::engine
        H2[Perceptual Hash]:::engine
        H3[Graph Matching]:::engine
        I[Storage Layer]:::storage
        J1[Sled DB]:::storage
        J2[File System]:::storage
        J3[Vector DB]:::storage
        
        A --> B
        B --> C
        C --> D1 & D2 & D3 & D4
        D1 & D2 & D3 & D4 --> E
        E --> F
        B --> G
        G --> H1 & H2 & H3
        H1 & H2 & H3 --> I
        I --> J1 & J2 & J3
    end

    style Core fill:#f8f9fa,stroke:#2b7489,stroke-width:2px
```

## 🌟 Key Features

- **Smart Classification**: AI-powered detection and routing of different data types
- **Multi-Modal Deduplication**: Specialized engines for text, JSON, images, and binary data
- **ML-Enhanced Matching**: Combines exact and fuzzy matching using advanced AI models
- **Intelligent Conflict Resolution**: Machine learning-assisted conflict handling with human oversight
- **High-Performance Search**: Vector-based similarity search using FAISS HNSW
- **Flexible Storage**: Pluggable storage backends supporting Sled DB, file system, and vector databases

## 🏗️ Architecture

### Frontend (Tauri + Vue)
- Modern, responsive UI for data visualization and management
- AI confidence score displays and similarity heatmaps
- Interactive conflict resolution interface
- Export and download capabilities

### Backend (Rust + AI)
- **Deduplication Engine**:
  - ML Text Deduper
  - GNN JSON Deduper
  - CNN Image Deduper
  - Deep Learning Binary Deduper
  
- **Smart Indexing**:
  - FAISS HNSW Vector Search
  - Perceptual Hashing
  - Graph Matching Algorithms

- **ML Conflict Resolution**:
  - Confidence-weighted decision making
  - Automated threshold tuning
  - Human-in-the-loop review system

### Storage Options
- Sled DB for high-performance local storage
- File system integration for simple deployments
- Vector DB support (Weaviate/Pinecone) for advanced similarity search

## 🚀 Getting Started

[Coming Soon]

## 🛠️ Development

[Coming Soon]

## 📚 Documentation

[Coming Soon]

## 🤝 Contributing

[Coming Soon]

## 📄 License

[Coming Soon]

## 🔗 Links

[Coming Soon]