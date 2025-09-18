package api

import (
    "net/http"

    "gin/config"
    "gin/middleware"
    "gin/models"
    "gin/services"

    "github.com/gin-gonic/gin"
)

type ResourceRequest struct {
    Title    string `json:"title" binding:"required"`
    Content  string `json:"content" binding:"required"`
    Language string `json:"language"`
    Format   string `json:"format"`
}

// POST /resources
func CreateResource(c *gin.Context) {
    db := config.GetDB()

    userID, ok := middleware.GetUserIDFromContext(c)
    if !ok {
        c.JSON(http.StatusUnauthorized, ErrorResponse{Error: "unauthorized", Message: "User not authenticated"})
        return
    }

    var req ResourceRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        c.JSON(http.StatusBadRequest, ErrorResponse{Error: "invalid_input", Message: "Invalid resource payload"})
        return
    }

    cur := services.NewCurationService()
    result := cur.CurateContent(req.Title, req.Content, req.Language, req.Format)

    resource := models.Resource{
        Title:     req.Title,
        Content:   req.Content,
        Language:  req.Language,
        Format:    req.Format,
        CreatorID: userID,
        Status:    string(result.Status),
    }

    if err := db.Create(&resource).Error; err != nil {
        c.JSON(http.StatusInternalServerError, ErrorResponse{Error: "database_error", Message: "Failed to create resource"})
        return
    }

    c.JSON(http.StatusCreated, gin.H{
        "data":    resource,
        "message": "Resource created",
        "curation": result,
    })
}

// PUT /resources/:id
func UpdateResource(c *gin.Context) {
    db := config.GetDB()

    userID, ok := middleware.GetUserIDFromContext(c)
    if !ok {
        c.JSON(http.StatusUnauthorized, ErrorResponse{Error: "unauthorized", Message: "User not authenticated"})
        return
    }

    id := c.Param("id")
    var resource models.Resource
    if err := db.First(&resource, id).Error; err != nil {
        c.JSON(http.StatusNotFound, ErrorResponse{Error: "not_found", Message: "Resource not found"})
        return
    }

    // Optional: only creator can update
    if resource.CreatorID != userID {
        c.JSON(http.StatusForbidden, ErrorResponse{Error: "forbidden", Message: "You cannot update this resource"})
        return
    }

    var req ResourceRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        c.JSON(http.StatusBadRequest, ErrorResponse{Error: "invalid_input", Message: "Invalid resource payload"})
        return
    }

    cur := services.NewCurationService()
    result := cur.CurateContent(req.Title, req.Content, req.Language, req.Format)

    updates := map[string]interface{}{
        "title":   req.Title,
        "content": req.Content,
        "language": req.Language,
        "format":  req.Format,
        "status":  string(result.Status),
    }

    if err := db.Model(&resource).Updates(updates).Error; err != nil {
        c.JSON(http.StatusInternalServerError, ErrorResponse{Error: "database_error", Message: "Failed to update resource"})
        return
    }

    // Reload updated resource
    if err := db.First(&resource, id).Error; err == nil {
        // continue
    }

    c.JSON(http.StatusOK, gin.H{
        "data":    resource,
        "message": "Resource updated",
        "curation": result,
    })
}
