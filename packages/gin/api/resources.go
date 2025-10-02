package api

import (
    "net/http"

    "gin/common"
    "gin/config"
    "gin/middleware"
    "gin/models"

    "github.com/gin-gonic/gin"
)

// ResourceCreateRequest defines the expected payload for creating a resource
type ResourceCreateRequest struct {
    Title    string `json:"title" binding:"required,min=1,max=200"`
    Content  string `json:"content" binding:"required,min=1"`
    Language string `json:"language" binding:"required,min=2,max=10"`
    Format   string `json:"format" binding:"required,min=2,max=50"`
    Theme    string `json:"theme" binding:"required,min=2,max=100"`
    Level    string `json:"level" binding:"required,min=1,max=50"`
}

// CreateResource handles educator-only creation of resources
func CreateResource(c *gin.Context) {
    // Role check: only Educator can create resources
    role, ok := middleware.GetUserRoleFromContext(c)
    if !ok {
        common.JSONError(c, http.StatusUnauthorized, "unauthorized", "User not authenticated")
        return
    }
    if role != "Educator" {
        common.JSONError(c, http.StatusForbidden, "forbidden", "Only Educators can create resources")
        return
    }

    var req ResourceCreateRequest
    if err := c.ShouldBindJSON(&req); err != nil {
        common.JSONError(c, http.StatusBadRequest, "invalid_input", "Invalid resource data provided")
        return
    }

    // Get authenticated user id
    userID, ok := middleware.GetUserIDFromContext(c)
    if !ok {
        common.JSONError(c, http.StatusUnauthorized, "unauthorized", "User ID not found in context")
        return
    }

    db := config.GetDB()

    resource := models.Resource{
        Title:     req.Title,
        Content:   req.Content,
        Language:  req.Language,
        Format:    req.Format,
        Theme:     req.Theme,
        Level:     req.Level,
        CreatorID: userID,
    }

    if err := db.Create(&resource).Error; err != nil {
        common.JSONError(c, http.StatusInternalServerError, "database_error", "Failed to create resource in database")
        return
    }

    common.JSONSuccess(c, http.StatusCreated, resource, "Resource created successfully")
}